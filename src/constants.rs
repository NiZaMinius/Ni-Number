//! Pre-computed constants and formatting utilities for the Ni number.

/// The Ni constant η_ν at `f64` precision.
///
/// Accurate to ~15–16 significant decimal digits.
/// For more digits, use [`crate::ni_number`] or [`crate::ni_number_digits`].
pub const NI_F64: f64 = 1.8893766604049191_f64;

/// The Ni constant η_ν at `f32` precision.
pub const NI_F32: f32 = 1.8893767_f32;

/// The first 50 decimal digits of η_ν (verified by this crate's own computation).
///
/// Can be used for quick validation or display purposes.
pub const NI_50_DIGITS: &str = "1.88937666040491913115597775087642096081019761538215";

/// Format a [`rug::Float`] to exactly `decimal_digits` digits after the decimal point.
///
/// Returns a `String` like `"1.889376506201356..."`.
pub fn format_digits(value: &rug::Float, decimal_digits: u32) -> String {
    // rug's to_string_radix gives us precise control
    // We use base 10, and request enough significant figures
    let sig_figs = decimal_digits + 2; // +2 for the integer part digits
    value.to_string_radix_round(10, Some(sig_figs as usize), rug::float::Round::Nearest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ni_f64_value() {
        // Sanity-check against independently computed value
        assert!((NI_F64 - 1.8893767).abs() < 1e-7);
    }

    #[test]
    fn test_ni_50_digits_prefix() {
        assert!(NI_50_DIGITS.starts_with("1.889376660"));
    }
}
