//! Lazy iterator over the terms of the η_ν series.
//!
//! Each step yields a [`NiStep`] containing the term index `n`,
//! the current term value, and the running partial sum.
//!
//! Useful for visualising convergence or stopping early at a desired
//! precision without computing the full series.
//!
//! ## Example
//!
//! ```rust,ignore
//! use ni_number::ni_series;
//!
//! for step in ni_series(128).take(10) {
//!     println!("n={:2}  term={:.6e}  sum={:.15}",
//!              step.n, step.term.to_f64(), step.sum.to_f64());
//! }
//! ```

use crate::backend::{NiBackend, NiFloat};

// ─── Step ─────────────────────────────────────────────────────────────────────

/// One step of the η_ν series.
pub struct NiStep<F: NiFloat> {
    /// Term index (1-based).
    pub n: u64,
    /// Value of the n-th term: `πⁿ / (n! · 2^(n²))`.
    pub term: F,
    /// Partial sum up to and including term `n`.
    pub sum: F,
}

// ─── Iterator ─────────────────────────────────────────────────────────────────

/// Lazy iterator over the terms of the η_ν series.
///
/// Created by [`crate::ni_series`].
pub struct NiSeries<B: NiBackend> {
    precision_bits: u32,
    n: u64,
    /// Accumulated π^n
    pi_pow: B::Float,
    /// Accumulated n!
    fact: B::Float,
    /// Accumulated 2^(n²)
    two_pow: B::Float,
    /// Running partial sum
    sum: B::Float,
    /// π (stored once)
    pi: B::Float,
}

impl<B: NiBackend> NiSeries<B> {
    /// Create a new iterator at the given bit precision.
    pub fn new(precision_bits: u32) -> Self {
        let pi = B::pi(precision_bits);
        let pi_pow = pi.clone(); // π^1
        let fact = B::from_u64(1, precision_bits); // 1!
        let two_pow = B::from_u64(2, precision_bits); // 2^(1²)
        let sum = B::zero(precision_bits);

        NiSeries {
            precision_bits,
            n: 0,
            pi_pow,
            fact,
            two_pow,
            sum,
            pi,
        }
    }
}

impl<B: NiBackend> Iterator for NiSeries<B> {
    type Item = NiStep<B::Float>;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.precision_bits;
        self.n += 1;

        // On the first step (n=1) the running products are already initialised.
        // On subsequent steps we advance them.
        if self.n > 1 {
            // π^n = π^(n-1) · π
            self.pi_pow = B::mul(&self.pi_pow, &self.pi, p);
            // n! = (n-1)! · n
            let n_f = B::from_u64(self.n, p);
            self.fact = B::mul(&self.fact, &n_f, p);
            // 2^(n²) = 2^((n-1)²) · 2^(2n-1)
            let step = B::two_pow(2 * self.n as u32 - 1, p);
            self.two_pow = B::mul(&self.two_pow, &step, p);
        }

        // term = π^n / (n! · 2^(n²))
        let denom = B::mul(&self.fact, &self.two_pow, p);
        let term = B::div(&self.pi_pow, &denom, p);

        self.sum.add_assign(&term);

        Some(NiStep {
            n: self.n,
            term: term.clone(),
            sum: self.sum.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "backend-dashu")]
    #[test]
    fn series_first_term_is_positive() {
        use crate::backend::dashu::DashuBackend;
        let step = NiSeries::<DashuBackend>::new(128).next().unwrap();
        println!("term f64 = {}", step.term.to_f64());
        println!("term str = {}", step.term.to_decimal_string(10));
        assert!(step.term.to_f64() > 0.0);
    }

    #[cfg(feature = "backend-dashu")]
    #[test]
    fn series_converges_to_known_value() {
        use crate::backend::dashu::DashuBackend;
        use crate::constants::NI_F64;
        // After 20 terms the partial sum should be extremely close to η_ν
        let sum = NiSeries::<DashuBackend>::new(256)
            .take(20)
            .last()
            .unwrap()
            .sum
            .to_f64();
        assert!(
            (sum - NI_F64).abs() < 1e-14,
            "series drift: {}",
            (sum - NI_F64).abs()
        );
    }
}
