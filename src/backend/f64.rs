//! Native `f64` backend — no dependencies, ~15 significant digits.
//!
//! Enable with `features = ["backend-f64"]`.
//!
//! ## When to use
//!
//! - Embedded targets or WASM where arbitrary-precision crates are too heavy
//! - Quick sanity checks during development
//! - Cases where 15-digit precision is genuinely sufficient
//!
//! ## Limitations
//!
//! IEEE 754 `f64` has 53 bits of mantissa (~15–16 decimal digits).
//! Beyond that point rounding errors accumulate and results are unreliable.
//! For any physics calculation involving sub-Planck distances, use
//! [`backend-dashu`](super::dashu) or [`backend-rug`](super::rug_backend) instead.

#![cfg(feature = "backend-f64")]

use crate::backend::{NiBackend, NiFloat};

// ─── Float wrapper ────────────────────────────────────────────────────────────

/// Thin wrapper around a native `f64`.
#[derive(Clone, Copy)]
pub struct F64Float(pub f64);

impl NiFloat for F64Float {
    #[inline]
    fn to_f64(&self) -> f64 {
        self.0
    }

    fn to_decimal_string(&self, digits: u32) -> String {
        format!("{:.prec$}", self.0, prec = digits as usize)
    }

    #[inline]
    fn abs(&self) -> Self {
        F64Float(self.0.abs())
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    #[inline]
    fn add_assign(&mut self, other: &Self) {
        self.0 += other.0;
    }
}

// ─── Backend ──────────────────────────────────────────────────────────────────

/// Native `f64` backend (~15 significant digits, zero dependencies).
pub struct F64Backend;

impl NiBackend for F64Backend {
    type Float = F64Float;

    #[inline]
    fn pi(_precision_bits: u32) -> F64Float {
        F64Float(std::f64::consts::PI)
    }

    #[inline]
    fn mul(a: &F64Float, b: &F64Float, _precision_bits: u32) -> F64Float {
        F64Float(a.0 * b.0)
    }

    #[inline]
    fn div(num: &F64Float, den: &F64Float, _precision_bits: u32) -> F64Float {
        F64Float(num.0 / den.0)
    }

    #[inline]
    fn from_u64(value: u64, _precision_bits: u32) -> F64Float {
        F64Float(value as f64)
    }

    #[inline]
    fn two_pow(exp: u32, _precision_bits: u32) -> F64Float {
        // f64::powi is exact for powers of 2 within range
        F64Float(2.0_f64.powi(exp as i32))
    }

    #[inline]
    fn zero(_precision_bits: u32) -> F64Float {
        F64Float(0.0)
    }

    fn epsilon(_precision_bits: u32) -> F64Float {
        // f64 machine epsilon — anything smaller is lost in rounding
        F64Float(f64::EPSILON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute;
    use crate::constants::NI_F64;
    use crate::backend::NiFloat;

    #[test]
    fn f64_backend_matches_constant() {
        let val = compute::compute::<F64Backend>(64); // precision_bits ignored by f64
        let diff = (val.to_f64() - NI_F64).abs();
        // f64 gives ~15 digits; allow generous tolerance
        assert!(diff < 1e-13, "f64 backend drift: {:.2e}", diff);
    }

    #[test]
    fn f64_string_prefix() {
        let val = compute::compute::<F64Backend>(64);
        let s = val.to_decimal_string(10);
        assert!(s.starts_with("1.8893766604"), "got: {}", s);
    }
}
