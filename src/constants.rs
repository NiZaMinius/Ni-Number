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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_ni_f64_value() {
        // Sanity-check against independently computed value
        assert!((NI_F64 - 1.8893767).abs() < 1e-7);
    }

    #[test]
    #[serial]
    fn test_ni_50_digits_prefix() {
        assert!(NI_50_DIGITS.starts_with("1.889376660"));
    }

    #[cfg(feature = "backend-dashu")]
    #[test]
    #[serial]
    fn verify_50_digits_exact() {
        use crate::backend::NiFloat;
        use crate::backend::dashu::DashuBackend;

        // We calculate with a reserve (100 digits = ~396 bits) to avoid rounding error at the end
        let bits = compute::digits_to_bits(100);
        let val = compute::compute::<DashuBackend>(bits);

        // We format it to exactly 50 characters.
        let computed_str = val.to_decimal_string(50);

        // We compare the calculated value with the hardcoded one
        assert_eq!(
            computed_str, NI_50_DIGITS,
            "\nEXPECTED: {}\nCOMPUTED: {}",
            NI_50_DIGITS, computed_str
        );
    }
}
