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

use rug::{Float, Integer};
use rug::ops::Pow;

/// Compute the Ni constant to the given precision in **bits**.
///
/// `precision_bits` controls the internal MPFR precision.
/// For decimal digits, use: `bits = ceil(digits × log2(10)) + guard_bits`.
///
/// # Examples
/// ```
/// use ni_number::compute::compute_ni;
/// let eta = compute_ni(128);
/// println!("{}", eta);
/// ```
pub fn compute_ni(precision_bits: u32) -> Float {
    // Extra guard bits to absorb rounding errors from accumulation
    let working_bits = precision_bits + 64;

    // π with working precision
    let pi = Float::with_val(working_bits, rug::float::Constant::Pi);

    // Convergence threshold: 2^(-(precision_bits + 32))
    let epsilon = Float::with_val(working_bits, 1.0)
        >> (precision_bits + 32);

    let mut sum = Float::with_val(working_bits, 0.0);

    // Incremental values kept across iterations for efficiency
    let mut pi_pow = Float::with_val(working_bits, &pi); // π^n, starts at π^1
    let mut factorial = Integer::from(1u32);             // n!, starts at 1! = 1

    for n in 1u32.. {
        // 2^(n²) as exact integer, then converted to Float
        let n_sq = n * n;
        let two_pow_n2: Integer = Integer::from(1u32) << n_sq;
        let two_pow_n2_f = Float::with_val(working_bits, &two_pow_n2);

        // term = π^n / (n! · 2^(n²))
        let denom = Float::with_val(working_bits, &factorial) * &two_pow_n2_f;
        let term = Float::with_val(working_bits, &pi_pow) / denom;

        sum += &term;

        // Check convergence
        if term.abs() < epsilon {
            break;
        }

        // Prepare next iteration: π^(n+1) and (n+1)!
        pi_pow *= &pi;
        factorial *= n + 1;
    }

    // Round down to requested precision before returning
    Float::with_val(precision_bits, sum)
}

/// Compute a single term of the series for index `n`.
///
/// Useful for inspecting convergence or educational purposes.
pub fn compute_term(n: u32, precision_bits: u32) -> Float {
    let working_bits = precision_bits + 64;

    let pi = Float::with_val(working_bits, rug::float::Constant::Pi);
    let pi_pow = Float::with_val(working_bits, pi.pow(n));

    let mut factorial = Integer::from(1u32);
    for k in 2..=n {
        factorial *= k;
    }

    let n_sq = n * n;
    let two_pow_n2: Integer = Integer::from(1u32) << n_sq;

    let denom = Float::with_val(working_bits, &factorial)
        * Float::with_val(working_bits, &two_pow_n2);

    Float::with_val(precision_bits, pi_pow / denom)
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

    /// Known value at f64 precision for regression testing
    const NI_F64_EXPECTED: f64 = 1.8893766604049191;

    #[test]
    fn test_basic_convergence() {
        let result = compute_ni(64);
        let as_f64: f64 = result.to_f64();
        let diff = (as_f64 - NI_F64_EXPECTED).abs();
        assert!(diff < 1e-10, "Result {} differs from expected {} by {}", as_f64, NI_F64_EXPECTED, diff);
    }

    #[test]
    fn test_digits_to_bits() {
        // 50 decimal digits needs at least 166 bits + 64 guard = 230
        assert!(digits_to_bits(50) >= 166);
    }

    #[test]
    fn test_term_n1() {
        // n=1: π / (1! · 2^1) = π/2 ≈ 1.5707963...
        let term = compute_term(1, 128);
        let as_f64: f64 = term.to_f64();
        let expected = std::f64::consts::PI / 2.0;
        assert!((as_f64 - expected).abs() < 1e-14);
    }

    #[test]
    fn test_term_n2() {
        // n=2: π² / (2! · 2^4) = π²/32 ≈ 0.3084251...
        let term = compute_term(2, 128);
        let as_f64: f64 = term.to_f64();
        let expected = std::f64::consts::PI * std::f64::consts::PI / 32.0;
        assert!((as_f64 - expected).abs() < 1e-14);
    }

    #[test]
    fn test_high_precision_starts_correctly() {
        // 100-digit computation must start with the known f64 digits
        let result = compute_ni(digits_to_bits(100));
        let s = format!("{:.15}", result);
        // Should start with 1.889376506201356...
        assert!(s.starts_with("1.889376660"), "Got: {}", s);
    }
}
