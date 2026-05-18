//! # ni-number
//!
//! High-precision computation of the **Ni constant** (η_ν) — the quantum energy
//! scattering constant proposed by NiZaMinius.
//!
//! ## Definition
//!
//! The Ni constant is defined by the series:
//!
//! ```text
//! η_ν = Σ (n=1..∞)  πⁿ / (n! · 2^(n²))
//! ```
//!
//! Its value begins: **1.88937666040491913115597775087642096081019761538215...**
//!
//! The series converges extremely fast — fewer than 60 terms are required
//! for thousands of decimal digits — making it practical even on low-end hardware.
//!
//! ## Quick Start
//!
//! ```rust
//! use ni_number::{NI_F64, ni_number_digits};
//!
//! // Fast constant at f64 precision
//! println!("η_ν ≈ {}", NI_F64);
//!
//! // 100 decimal digits
//! let digits = ni_number_digits(100);
//! println!("η_ν = {}", digits);
//! ```
//!
//! ## Feature flags
//!
//! This crate depends on [`rug`](https://crates.io/crates/rug), which links
//! against GNU GMP and MPFR. Ensure those system libraries are installed
//! (see README for OS-specific instructions).

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod compute;
pub mod constants;

pub use constants::{NI_50_DIGITS, NI_F32, NI_F64};

use rug::Float;

// ─── Primary public API ──────────────────────────────────────────────────────

/// Compute η_ν (the Ni constant) to the given precision in **bits**.
///
/// Returns a [`rug::Float`] that can be inspected, formatted, or used
/// in further arbitrary-precision calculations.
///
/// # Arguments
///
/// * `precision_bits` — Internal MPFR bit precision. Use [`bits_for_digits`]
///   to convert from decimal digits.
///
/// # Examples
///
/// ```rust
/// use ni_number::{ni_number, bits_for_digits};
///
/// // 200 decimal digits
/// let eta = ni_number(bits_for_digits(200));
/// println!("{:.200}", eta);
/// ```
pub fn ni_number(precision_bits: u32) -> Float {
    compute::compute_ni(precision_bits)
}

/// Compute η_ν and return a decimal string with exactly `decimal_digits`
/// digits after the decimal point.
///
/// This is the most convenient function for displaying or storing the constant.
///
/// # Arguments
///
/// * `decimal_digits` — Number of digits after the decimal point.
///
/// # Examples
///
/// ```rust
/// use ni_number::ni_number_digits;
///
/// let s = ni_number_digits(50);
/// assert!(s.starts_with("1.889376660"));
/// println!("η_ν = {}", s);
/// ```
pub fn ni_number_digits(decimal_digits: u32) -> String {
    let bits = compute::digits_to_bits(decimal_digits);
    let value = compute::compute_ni(bits);
    constants::format_digits(&value, decimal_digits)
}

/// Compute a single term of the defining series at index `n`.
///
/// Useful for studying convergence or teaching the mathematical structure.
///
/// # Examples
///
/// ```rust
/// use ni_number::ni_term;
///
/// // n=1: π / 2 ≈ 1.5707...
/// let t1 = ni_term(1, 64);
/// println!("term(1) = {:.10}", t1);
/// ```
pub fn ni_term(n: u32, precision_bits: u32) -> Float {
    compute::compute_term(n, precision_bits)
}

/// Convert a number of desired decimal digits to the required bit precision.
///
/// Adds 64 guard bits to protect against accumulated rounding errors.
///
/// # Examples
///
/// ```rust
/// use ni_number::bits_for_digits;
///
/// assert!(bits_for_digits(100) > 332); // 100 * log2(10) ≈ 332 + 64 guard
/// ```
pub fn bits_for_digits(decimal_digits: u32) -> u32 {
    compute::digits_to_bits(decimal_digits)
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ni_number_digits_prefix() {
        let s = ni_number_digits(30);
        assert!(s.starts_with("1.889376660"), "Unexpected output: {}", s);
    }

    #[test]
    fn test_ni_f64_close_to_computed() {
        let computed: f64 = ni_number(128).to_f64();
        let diff = (computed - NI_F64).abs();
        assert!(diff < 1e-14, "Drift: {}", diff);
    }

    #[test]
    fn test_term_sum_converges() {
        // Sum of first 20 terms should already match NI_F64 to 15 digits
        use rug::Float;
        let mut sum = Float::with_val(128, 0.0);
        for n in 1..=20 {
            sum += ni_term(n, 128);
        }
        let diff = (sum.to_f64() - NI_F64).abs();
        assert!(diff < 1e-14, "Sum after 20 terms: drift = {}", diff);
    }
}
