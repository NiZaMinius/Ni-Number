//! Pure-Rust backend using the [`dashu`] crate.
//!
//! This is the **default backend** — works on every platform
//! without any system dependencies.
//!
//! π is computed internally via Machin's formula using integer arithmetic:
//! π/4 = 4·arctan(1/5) − arctan(1/239)
//!
//! This converges rapidly and is computed entirely with `dashu`'s
//! arbitrary-precision integers — no floating-point instability.

#![cfg(feature = "backend-dashu")]

use crate::backend::{NiBackend, NiFloat};
use dashu::float::{FBig, round::mode::HalfAway};
use dashu::integer::IBig;
use std::collections::BTreeMap;
use std::sync::{OnceLock, RwLock};

/// Precision context type with explicit rounding mode.
type Ctx = dashu::float::Context<HalfAway>;

// ─── Float wrapper ────────────────────────────────────────────────────────────

/// Global π cache — computed once per precision level, reused across calls.
static PI_CACHE: OnceLock<RwLock<BTreeMap<u32, FBig<HalfAway>>>> = OnceLock::new();

fn pi_cache() -> &'static RwLock<BTreeMap<u32, FBig<HalfAway>>> {
    PI_CACHE.get_or_init(|| RwLock::new(BTreeMap::new()))
}

/// Wrapper around `dashu::float::FBig`.
#[derive(Clone)]
pub struct DashuFloat {
    pub(crate) inner: FBig<HalfAway>,
}

impl NiFloat for DashuFloat {
    fn to_f64(&self) -> f64 {
        // dashu requires owned value for TryFrom conversion
        let s = self
            .inner
            .clone()
            .with_base_and_precision::<10>(20)
            .value()
            .to_string();
        s.parse::<f64>().unwrap_or(f64::NAN)
    }

    fn to_decimal_string(&self, digits: u32) -> String {
        // Special case: 0 digits after decimal point — return rounded integer
        if digits == 0 {
            let decimal = self.inner.clone().with_base_and_precision::<10>(5).value();
            let s = decimal.to_string();
            // Take only the integer part, rounded
            return if let Some(dot_pos) = s.find('.') {
                let int_part: u64 = s[..dot_pos].parse().unwrap_or(1);
                let first_decimal = s
                    .chars()
                    .nth(dot_pos + 1)
                    .and_then(|c| c.to_digit(10))
                    .unwrap_or(0);
                if first_decimal >= 5 {
                    (int_part + 1).to_string()
                } else {
                    int_part.to_string()
                }
            } else {
                s
            };
        }

        let target_prec = digits as usize;
        let decimal = self
            .inner
            .clone()
            .with_base_and_precision::<10>(target_prec + 20)
            .value();
        let s = decimal.to_string();

        if let Some(dot_pos) = s.find('.') {
            let end_of_digits = s.find('e').unwrap_or(s.len());
            let end = (dot_pos + 1 + target_prec).min(end_of_digits);
            let mut result = s[..end].to_string();
            let current_prec = result.len() - dot_pos - 1;
            if current_prec < target_prec {
                result.extend(std::iter::repeat('0').take(target_prec - current_prec));
            }
            result
        } else {
            let mut result = s;
            if target_prec > 0 {
                result.push('.');
                result.extend(std::iter::repeat('0').take(target_prec));
            }
            result
        }
    }

    fn abs(&self) -> Self {
        use dashu::base::Abs;
        DashuFloat {
            inner: self.inner.clone().abs(),
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    fn add_assign(&mut self, other: &Self) {
        use std::ops::AddAssign;
        self.inner.add_assign(other.inner.clone());
    }
}

// ─── Backend ──────────────────────────────────────────────────────────────────

/// Pure-Rust arbitrary-precision backend (dashu).
pub struct DashuBackend;

impl NiBackend for DashuBackend {
    type Float = DashuFloat;

    fn pi(precision_bits: u32) -> DashuFloat {
        // Round up to nearest 64 bits to share cache entries across nearby precisions
        let key = ((precision_bits + 63) / 64) * 64;

        // Fast path — read lock
        {
            let cache = pi_cache().read().unwrap();
            if let Some(cached) = cache.get(&key) {
                return DashuFloat {
                    inner: cached.clone(),
                };
            }
        }

        // Slow path — compute π via Machin's formula and store
        let digits = bits_to_decimal_digits(key);
        let ctx = make_ctx(key);
        let pi = machin_pi(&ctx, digits);

        {
            let mut cache = pi_cache().write().unwrap();
            cache.entry(key).or_insert_with(|| pi.clone());
        }

        DashuFloat { inner: pi }
    }

    fn mul(a: &DashuFloat, b: &DashuFloat, precision_bits: u32) -> DashuFloat {
        let ctx = make_ctx(precision_bits);
        DashuFloat {
            inner: ctx.mul(a.inner.repr(), b.inner.repr()).value(),
        }
    }

    fn div(num: &DashuFloat, den: &DashuFloat, precision_bits: u32) -> DashuFloat {
        let ctx = make_ctx(precision_bits);
        DashuFloat {
            inner: ctx.div(num.inner.repr(), den.inner.repr()).value(),
        }
    }

    fn from_u64(value: u64, precision_bits: u32) -> DashuFloat {
        let ctx = make_ctx(precision_bits);
        let int = IBig::from(value);
        DashuFloat {
            inner: ctx.convert_int(int).value(),
        }
    }

    fn two_pow(exp: u32, precision_bits: u32) -> DashuFloat {
        let ctx = make_ctx(precision_bits);
        // 2^exp via bit shift on IBig — exact, no float involved
        let int = IBig::from(1u64) << exp as usize;
        DashuFloat {
            inner: ctx.convert_int(int).value(),
        }
    }

    fn zero(_precision_bits: u32) -> DashuFloat {
        DashuFloat { inner: FBig::ZERO }
    }

    fn epsilon(precision_bits: u32) -> DashuFloat {
        // epsilon = 1 / 2^(precision_bits + 64)
        let exp = precision_bits + 64;
        let ctx = make_ctx(precision_bits);
        let one = ctx.convert_int(IBig::from(1u64)).value();
        let den = ctx.convert_int(IBig::from(1u64) << exp as usize).value();
        DashuFloat {
            inner: ctx.div(one.repr(), den.repr()).value(),
        }
    }
}

// ─── π via Machin's formula ───────────────────────────────────────────────────

/// Compute π using Machin's formula:
/// π/4 = 4·arctan(1/5) − arctan(1/239)
///
/// arctan(1/x) is computed as the alternating series:
/// arctan(1/x) = 1/x − 1/(3x³) + 1/(5x⁵) − ...
///
/// Everything is done in integer arithmetic scaled by `scale = 10^digits`
/// so no precision is lost in intermediate steps.
fn machin_pi(ctx: &Ctx, digits: usize) -> FBig<HalfAway> {
    // Scale factor: work in integers, divide at the end
    let scale = IBig::from(10u64).pow(digits + 10);

    let arctan5 = arctan_series(&scale, 5);
    let arctan239 = arctan_series(&scale, 239);

    // π/4 = 4·arctan(1/5) − arctan(1/239)
    let pi_scaled = arctan5 * IBig::from(4u64) * IBig::from(4u64) - arctan239 * IBig::from(4u64);

    // Convert scaled integer back to FBig
    let pi_int = ctx.convert_int(pi_scaled).value();
    let scale_f = ctx.convert_int(scale).value();
    ctx.div(pi_int.repr(), scale_f.repr()).value()
}

/// Compute arctan(1/x) × scale using the Gregory–Leibniz series.
/// Returns an integer (scaled result).
fn arctan_series(scale: &IBig, x: u64) -> IBig {
    let x_big = IBig::from(x);
    let x2 = x_big.clone() * x_big.clone();

    let mut term = scale / x_big.clone();
    let mut sum = term.clone();
    let mut sign = true; // первая итерация должна вычитать
    let mut k = 3u64;

    loop {
        term = &term / &x2;
        let contribution = &term / IBig::from(k);

        if contribution == IBig::ZERO {
            break;
        }

        if sign {
            sum -= contribution;
        } else {
            sum += contribution;
        }

        sign = !sign;
        k += 2;
    }

    sum
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Convert bit precision to decimal digit count with guard digits.
fn bits_to_decimal_digits(bits: u32) -> usize {
    ((bits as f64) / std::f64::consts::LOG2_10).ceil() as usize + 20 // +20 guard digits
}

fn make_ctx(precision_bits: u32) -> Ctx {
    Ctx::new(precision_bits as usize + 64) // +64 guard bits
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::NiFloat;
    use crate::compute;
    use crate::constants::NI_F64;

    #[test]
    fn pi_is_correct() {
        let bits = 128; // было 30 — слишком мало
        let ctx = make_ctx(bits);
        let digits = bits_to_decimal_digits(bits);
        let pi = machin_pi(&ctx, digits);
        let pi_f = DashuFloat { inner: pi };
        let diff = (pi_f.to_f64() - std::f64::consts::PI).abs();
        assert!(diff < 1e-14, "π drift: {:.2e}", diff);
    }

    #[test]
    fn ni_constant_matches_known_value() {
        let val = compute::compute::<DashuBackend>(256);
        let s = val.to_decimal_string(20);
        assert!(s.starts_with("1.88937666040491913"), "wrong value: {}", s);
    }

    #[test]
    fn ni_f64_matches_constant() {
        let val = compute::compute::<DashuBackend>(128);
        let diff = (val.to_f64() - NI_F64).abs();
        assert!(diff < 1e-13, "drift: {:.2e}", diff);
    }
}
