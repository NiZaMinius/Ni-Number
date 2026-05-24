//! # ni-number
//!
//! High-precision computation of the **Ni constant** (η_ν) —
//! the quantum energy scattering constant.
//!
//! ## Definition
//!
//! The Ni constant is defined by the series:
//!
//! ```text
//! η_ν = Σ (n=1..∞)  πⁿ / (n! · 2^(n²))
//! ```
//!
//! Value: **1.88937666040491913115597775087642096081019761538215...**
//!
//! ## Backends
//!
//! | Feature           | Backend   | Requires               | Precision  |
//! |-------------------|-----------|------------------------|------------|
//! | `backend-dashu`   | pure Rust | nothing **(default)**  | arbitrary  |
//! | `backend-rug`     | GNU MPFR  | MSYS2 on Windows       | arbitrary  |
//!
//! ## Quick start
//!
//! ```toml
//! # Cargo.toml — pure Rust, works on every platform
//! [dependencies]
//! ni-number = "1.0"
//!
//! # Maximum performance via GNU MPFR
//! ni-number = { version = "1.0", features = ["backend-rug"] }
//! ```
//!
//! ```rust,ignore
//! use ni_number::{NI_F64, ni_number_digits};
//!
//! println!("η_ν ≈ {}", NI_F64);                // fast — pre-computed constant
//! println!("η_ν = {}", ni_number_digits(100));  // 100 decimal digits
//! ```

#![warn(missing_docs)]

pub mod backend;
pub mod cache;
pub mod compute;
pub mod constants;
pub mod series;

pub use compute::digits_to_bits as bits_for_digits;
pub use constants::{NI_50_DIGITS, NI_F32, NI_F64};

// ─── Select active backend at compile time ───────────────────────────────────

// `backend-rug` wins when explicitly requested AND `backend-dashu` is NOT also
// the only default.  In practice: if the user adds `features = ["backend-rug"]`
// without disabling defaults they get rug (which supersedes dashu in our cfg).

#[cfg(all(feature = "backend-rug", not(feature = "backend-dashu")))]
type ActiveBackend = backend::rug_backend::RugBackend;

#[cfg(feature = "backend-dashu")]
type ActiveBackend = backend::dashu::DashuBackend;

#[cfg(all(
    feature = "backend-f64",
    not(feature = "backend-dashu"),
    not(feature = "backend-rug")
))]
type ActiveBackend = backend::f64_backend::F64Backend;

// ─── Global cache ─────────────────────────────────────────────────────────────

use cache::NiCache;

// SAFETY: NiCache uses an internal RwLock and is Send + Sync.
static CACHE: NiCache<ActiveBackend> = NiCache::new();

// ─── Public API ──────────────────────────────────────────────────────────────

/// Compute η_ν and return a decimal string with `decimal_digits` digits
/// after the decimal point.
///
/// The result is cached — repeated calls at the same (or lower) precision
/// return instantly without recomputing.
///
/// # Example
///
/// ```rust,ignore
/// use ni_number::ni_number_digits;
///
/// let s = ni_number_digits(50);
/// assert!(s.starts_with("1.889376660404919"));
/// ```
pub fn ni_number_digits(decimal_digits: u32) -> String {
    let bits = compute::digits_to_bits(decimal_digits);
    let val = CACHE.get_or_compute(bits);
    use backend::NiFloat;
    val.to_decimal_string(decimal_digits)
}

/// Compute η_ν to `precision_bits` of internal bit precision.
///
/// Returns the backend's native float type (opaque outside this crate).
/// Use [`bits_for_digits`] to convert from decimal digit count to bits.
///
/// Results are cached — repeated calls at the same precision are instant.
pub fn ni_number(precision_bits: u32) -> <ActiveBackend as backend::NiBackend>::Float {
    CACHE.get_or_compute(precision_bits)
}

/// Return a lazy iterator over the individual series terms.
///
/// Each item is a [`series::NiStep`] with fields `n`, `term`, and `sum`.
/// Useful for visualising convergence or stopping at a custom threshold.
///
/// # Example
///
/// ```rust,ignore
/// use ni_number::ni_series;
/// use ni_number::backend::NiFloat;
///
/// for step in ni_series(128).take(10) {
///     println!("n={:2}  sum={:.15}", step.n, step.sum.to_f64());
/// }
/// ```
pub fn ni_series(precision_bits: u32) -> series::NiSeries<ActiveBackend> {
    series::NiSeries::new(precision_bits)
}

/// Clear the internal cache and free its memory.
///
/// The next call to [`ni_number`] or [`ni_number_digits`] will recompute
/// from scratch.  Useful in long-running applications that need to reclaim
/// memory after a high-precision computation is no longer needed.
pub fn clear_cache() {
    CACHE.clear();
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use backend::NiFloat;
    use serial_test::serial;

    #[test]
    #[serial]
    fn digits_prefix_is_correct() {
        let s = ni_number_digits(20);
        assert!(s.starts_with("1.88937666040491913"), "wrong prefix: {}", s);
    }

    #[test]
    #[serial]
    fn f64_matches_precomputed_constant() {
        let computed = ni_number(128).to_f64();
        assert!(
            (computed - NI_F64).abs() < 1e-13,
            "drift: {:.2e}",
            (computed - NI_F64).abs()
        );
    }

    #[test]
    #[serial]
    fn cache_is_idempotent() {
        let a = ni_number(128).to_f64();
        let b = ni_number(128).to_f64();
        assert_eq!(a, b, "cache returned different values");
    }

    #[test]
    #[serial]
    fn series_converges_to_constant() {
        use crate::series::NiSeries;
        let sum = NiSeries::<ActiveBackend>::new(256)
            .take(20)
            .last()
            .unwrap()
            .sum
            .to_f64();
        assert!(
            (sum - NI_F64).abs() < 1e-13,
            "series drift: {:.2e}",
            (sum - NI_F64).abs()
        );
    }

    #[test]
    #[serial]
    fn clear_and_recompute_is_stable() {
        let _ = ni_number(64);
        clear_cache();
        let val = ni_number(64).to_f64();
        assert!((val - NI_F64).abs() < 1e-12);
    }

    mod edge_cases {
        use super::*;
        use serial_test::serial;

        #[test]
        #[serial]
        fn test_zero_digits() {
            // Check: 0 digits after the decimal point, 1.889... must be correctly rounded to 2.
            let s = ni_number_digits(0);
            assert_eq!(s, "2", "0 decimal digits should round to 2");
        }

        #[test]
        #[serial]
        fn test_massive_cache_jump() {
            // Check: A sharp jump in accuracy should not break the cache
            clear_cache();
            let _ = ni_number_digits(10);
            let high = ni_number_digits(1000);
            assert!(high.starts_with("1.88937666040491913115597775087642096081019761538215"));
        }
    }
}
