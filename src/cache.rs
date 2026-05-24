//! Thread-safe cache for computed Ni constant values.
//!
//! Computing η_ν is expensive (arbitrary-precision arithmetic).
//! This module ensures the work is done at most once per precision level.
//!
//! ## Design
//!
//! - `NiCache<B>` is a `static`-friendly wrapper around a `RwLock<BTreeMap<…>>`.
//! - Keys are `precision_bits` rounded up to multiples of 64 —
//!   so a request for 100 bits reuses a cached 128-bit result.
//! - `const fn new()` is `stable` since Rust 1.63, so `NiCache` can be
//!   used in a `static` without `lazy_static` or `once_cell`.

use crate::backend::NiBackend;
use crate::compute;
use std::collections::BTreeMap;
use std::sync::RwLock;

/// Thread-safe cache for η_ν values at different precisions.
pub struct NiCache<B: NiBackend> {
    /// Maps `rounded_bits` → computed value.
    map: RwLock<BTreeMap<u32, B::Float>>,
}

impl<B: NiBackend> NiCache<B> {
    /// Create a new, empty cache.
    ///
    /// Suitable for use in a `static` initialiser.
    pub const fn new() -> Self {
        // RwLock::new is const since Rust 1.63
        NiCache {
            map: RwLock::new(BTreeMap::new()),
        }
    }

    /// Return a cached value if one exists at sufficient precision,
    /// otherwise compute and store it.
    ///
    /// The cache key is `precision_bits` rounded up to the nearest 64
    /// so nearby precisions share results.
    pub fn get_or_compute(&self, precision_bits: u32) -> B::Float {
        let key = round_up(precision_bits, 64);

        // Fast path — read lock
        {
            let map = self.map.read().expect("NiCache RwLock poisoned");
            if let Some(cached) = map.get(&key) {
                return cached.clone();
            }
        }

        // Slow path — compute then write
        let value = compute::compute::<B>(key);

        {
            let mut map = self.map.write().expect("NiCache RwLock poisoned");
            // Double-checked: another thread may have computed while we did
            map.entry(key).or_insert_with(|| value.clone());
        }

        value
    }

    /// Clear all cached values.
    ///
    /// The next call to [`get_or_compute`](Self::get_or_compute)
    /// will recompute from scratch.
    pub fn clear(&self) {
        let mut map = self.map.write().expect("NiCache RwLock poisoned");
        map.clear();
    }
}

/// Round `n` up to the nearest multiple of `step`.
#[inline]
fn round_up(n: u32, step: u32) -> u32 {
    ((n + step - 1) / step) * step
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::NiFloat;

    #[test]
    fn round_up_works() {
        assert_eq!(round_up(1, 64), 64);
        assert_eq!(round_up(64, 64), 64);
        assert_eq!(round_up(65, 64), 128);
        assert_eq!(round_up(128, 64), 128);
    }

    #[cfg(feature = "backend-dashu")]
    #[test]
    fn cache_returns_same_value_twice() {
        use crate::backend::dashu::DashuBackend;
        let cache: NiCache<DashuBackend> = NiCache::new();
        let a = cache.get_or_compute(128).to_f64();
        let b = cache.get_or_compute(128).to_f64();
        assert_eq!(a, b);
    }

    #[cfg(feature = "backend-dashu")]
    #[test]
    fn cache_clears_correctly() {
        use crate::backend::NiFloat;
        use crate::backend::dashu::DashuBackend;
        let cache: NiCache<DashuBackend> = NiCache::new();
        let _ = cache.get_or_compute(128);
        cache.clear();
        // After clear, recompute should still give the same number
        let val = cache.get_or_compute(128).to_f64();
        assert!((val - 1.8893766604_f64).abs() < 1e-9);
    }
}
