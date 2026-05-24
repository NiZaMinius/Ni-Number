//! Backend trait definitions for the Ni constant computation engine.
//!
//! Every backend must implement [`NiBackend`] and [`NiFloat`].
//! The rest of the library is generic over these traits —
//! the algorithm, cache, and iterator never depend on a specific number type.

pub mod dashu;

#[cfg(feature = "backend-rug")]
pub mod rug;

#[cfg(feature = "backend-f64")]
pub mod f64;

// ─── Public trait: arbitrary-precision float ──────────────────────────────────

/// A trait for arbitrary-precision floating-point numbers.
///
/// Every backend wraps its own float type behind this interface so that
/// the rest of the library stays backend-agnostic.
pub trait NiFloat: Clone + Send + Sync + 'static {
    /// Convert to `f64` for display or quick comparisons.
    /// Precision beyond ~15 digits is lost.
    fn to_f64(&self) -> f64;

    /// Format as a decimal string with exactly `digits` digits after the point.
    fn to_decimal_string(&self, digits: u32) -> String;

    /// Return the absolute value.
    fn abs(&self) -> Self;

    /// Return `true` if this value is strictly less than `other`.
    fn lt(&self, other: &Self) -> bool;

    /// Add `other` into `self` in-place.
    fn add_assign(&mut self, other: &Self);
}

// ─── Public trait: computation backend ───────────────────────────────────────

/// A backend that can compute the Ni constant to arbitrary precision.
///
/// Implement this trait to plug in a new number library.
///
/// Shipped implementations:
/// - [`dashu`](self::dashu::DashuBackend) — pure Rust, default
/// - [`rug_backend`](self::rug_backend::RugBackend) — GNU MPFR, `features = ["backend-rug"]`
pub trait NiBackend: 'static {
    /// The float type this backend produces.
    type Float: NiFloat;

    /// Compute π to at least `precision_bits` of precision.
    fn pi(precision_bits: u32) -> Self::Float;

    /// Multiply two values and return the result at `precision_bits`.
    fn mul(a: &Self::Float, b: &Self::Float, precision_bits: u32) -> Self::Float;

    /// Divide `num / den` and return the result at `precision_bits`.
    fn div(num: &Self::Float, den: &Self::Float, precision_bits: u32) -> Self::Float;

    /// Convert an exact `u64` integer to this backend's float type.
    fn from_u64(value: u64, precision_bits: u32) -> Self::Float;

    /// Compute `2^exp` as a float (exact for reasonable exponents).
    fn two_pow(exp: u32, precision_bits: u32) -> Self::Float;

    /// Create a zero value at the given precision.
    fn zero(precision_bits: u32) -> Self::Float;

    /// Convergence epsilon: roughly `2^(-(precision_bits + 32))`.
    fn epsilon(precision_bits: u32) -> Self::Float;
}
