//! GNU MPFR backend using the [`rug`] crate.
//!
//! Enable with `features = ["backend-rug"]`.
//! Requires GNU GMP + MPFR system libraries.
//! See the README installation section for details.

#![cfg(feature = "backend-rug")]

use rug::{Float, Integer};
use crate::backend::{NiBackend, NiFloat};

// ─── Float wrapper ────────────────────────────────────────────────────────────

/// Wrapper around `rug::Float`.
#[derive(Clone)]
pub struct RugFloat {
    pub(crate) inner: Float,
}

impl NiFloat for RugFloat {
    #[inline]
    fn to_f64(&self) -> f64 {
        self.inner.to_f64()
    }

    fn to_decimal_string(&self, digits: u32) -> String {
        let sig = (digits + 5) as usize;
        let s = self.inner
            .to_string_radix_round(10, Some(sig), rug::float::Round::Nearest);

        // Обрезаем так же как в dashu — без округления
        if let Some(dot_pos) = s.find('.') {
            let end_of_digits = s.find('e').unwrap_or(s.len());
            let end = (dot_pos + 1 + digits as usize).min(end_of_digits);
            let mut result = s[..end].to_string();
            let current_prec = result.len() - dot_pos - 1;
            if current_prec < digits as usize {
                result.extend(std::iter::repeat('0').take(digits as usize - current_prec));
            }
            result
        } else {
            s
        }
    }

    fn abs(&self) -> Self {
        RugFloat {
            inner: Float::with_val(self.inner.prec(), self.inner.clone().abs()),
        }
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.inner < other.inner
    }

    #[inline]
    fn add_assign(&mut self, other: &Self) {
        self.inner += &other.inner;
    }
}

// ─── Backend ──────────────────────────────────────────────────────────────────

/// GNU MPFR arbitrary-precision backend.
pub struct RugBackend;

impl NiBackend for RugBackend {
    type Float = RugFloat;

    fn pi(precision_bits: u32) -> RugFloat {
        RugFloat {
            inner: Float::with_val(precision_bits, rug::float::Constant::Pi),
        }
    }

    fn mul(a: &RugFloat, b: &RugFloat, precision_bits: u32) -> RugFloat {
        RugFloat {
            inner: Float::with_val(precision_bits, &a.inner * &b.inner),
        }
    }

    fn div(num: &RugFloat, den: &RugFloat, precision_bits: u32) -> RugFloat {
        RugFloat {
            inner: Float::with_val(precision_bits, &num.inner / &den.inner),
        }
    }

    fn from_u64(value: u64, precision_bits: u32) -> RugFloat {
        RugFloat {
            inner: Float::with_val(precision_bits, value),
        }
    }

    fn two_pow(exp: u32, precision_bits: u32) -> RugFloat {
        let int = Integer::from(1u32) << exp;
        RugFloat {
            inner: Float::with_val(precision_bits, int),
        }
    }

    fn zero(precision_bits: u32) -> RugFloat {
        RugFloat {
            inner: Float::with_val(precision_bits, 0.0_f64),
        }
    }

    fn epsilon(precision_bits: u32) -> RugFloat {
        // 2^(-(precision_bits + 32))
        let shift = precision_bits + 32;
        let val = Float::with_val(precision_bits, 1.0_f64) >> shift;
        RugFloat { inner: val }
    }
}
