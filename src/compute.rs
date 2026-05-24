//! Core algorithm for computing the Ni constant (η_ν).
//!
//! The Ni constant is defined as:
//!
//! ```text
//! η_ν = Σ (n=1..∞)  πⁿ / (n! · 2^(n²))
//! ```
//!
//! The series converges extremely fast due to the 2^(n²) denominator.
//! In practice, fewer than 60 terms are needed for thousands of decimal digits.

use crate::backend::{NiBackend, NiFloat};
use crate::series::NiSeries;

/// Compute the Ni constant to the given precision in **bits**.
///
/// `precision_bits` controls the internal backend precision.
/// For decimal digits, use: `bits = ceil(digits × log2(10)) + guard_bits`.
pub fn compute<B: NiBackend>(precision_bits: u32) -> B::Float {
    // Extra guard bits to absorb rounding errors from accumulation
    let working_bits = precision_bits + 64;
    let epsilon = B::epsilon(working_bits);

    let mut final_sum = B::zero(working_bits);

    // Use the optimized abstract iterator to compute the sum
    for step in NiSeries::<B>::new(working_bits) {
        final_sum = step.sum;
        if step.term.abs().lt(&epsilon) {
            break;
        }
    }

    // Return the final accumulated sum
    final_sum
}

/// Compute a single term of the series for index `n`.
///
/// Useful for inspecting convergence or educational purposes.
pub fn compute_term<B: NiBackend>(n: u32, precision_bits: u32) -> B::Float {
    let working_bits = precision_bits + 64;
    let mut series = NiSeries::<B>::new(working_bits);
    let mut final_term = B::zero(working_bits);

    for _ in 0..n {
        if let Some(step) = series.next() {
            final_term = step.term;
        } else {
            break;
        }
    }

    final_term
}

/// Convert a desired number of decimal digits to the required bit precision.
///
/// Uses the relation: 1 decimal digit ≈ log2(10) ≈ 3.32193 bits.
pub fn digits_to_bits(decimal_digits: u32) -> u32 {
    // ceil(digits * log2(10)) + 64 guard bits
    let bits = (decimal_digits as f64 * std::f64::consts::LOG2_10).ceil() as u32;
    bits + 64
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    /// Known value at f64 precision for regression testing
    const NI_F64_EXPECTED: f64 = 1.8893766604049191;

    #[cfg(feature = "backend-dashu")]
    #[test]
    #[serial]
    fn test_basic_convergence() {
        use crate::backend::dashu::DashuBackend;
        let result = compute::<DashuBackend>(64);
        let as_f64: f64 = result.to_f64();
        let diff = (as_f64 - NI_F64_EXPECTED).abs();
        assert!(
            diff < 1e-10,
            "Result {} differs from expected {} by {}",
            as_f64,
            NI_F64_EXPECTED,
            diff
        );
    }

    #[test]
    #[serial]
    fn test_digits_to_bits() {
        // 50 decimal digits needs at least 166 bits + 64 guard = 230
        assert!(digits_to_bits(50) >= 166);
    }

    #[cfg(feature = "backend-dashu")]
    #[test]
    #[serial]
    fn test_term_n1() {
        use crate::backend::dashu::DashuBackend;
        // n=1: π / (1! · 2^1) = π/2 ≈ 1.5707963...
        let term = compute_term::<DashuBackend>(1, 128);
        let as_f64: f64 = term.to_f64();
        let expected = std::f64::consts::PI / 2.0;
        assert!((as_f64 - expected).abs() < 1e-14);
    }

    #[cfg(feature = "backend-dashu")]
    #[test]
    #[serial]
    fn test_term_n2() {
        use crate::backend::dashu::DashuBackend;
        // n=2: π² / (2! · 2^4) = π²/32 ≈ 0.3084251...
        let term = compute_term::<DashuBackend>(2, 128);
        let as_f64: f64 = term.to_f64();
        let expected = std::f64::consts::PI * std::f64::consts::PI / 32.0;
        assert!((as_f64 - expected).abs() < 1e-14);
    }
}
