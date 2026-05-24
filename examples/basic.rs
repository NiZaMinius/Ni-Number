//! Basic usage example for the `ni-number` crate.
//!
//! Run with: `cargo run --example basic`
use ni_number::{NI_F64, NI_50_DIGITS, ni_number_digits, ni_series};
use ni_number::backend::NiFloat;
fn main() {
    println!("══════════════════════════════════════════════");
    println!("  Ni Constant (η_ν) — Basic Ni Constant Usage");
    println!("══════════════════════════════════════════════\n");

    // 1. Fast f64 constants (without calculations)
    println!("Fast f64:\n{:.15}\n", NI_F64);

    // 2. String constant 50 characters
    println!("Static 50 digits:\n{}\n", NI_50_DIGITS);

    // 3. On-the-fly calculation (result is cached)
    let custom_digits = ni_number_digits(150);
    println!("Computed 150 digits:\n{}\n", custom_digits);

    // 4. Viewing the convergence of a series (first 5 steps)
    println!("Series convergence (first 5 steps):");
    for step in ni_series(128).take(5) {
        println!("  n = {}: {}", step.n, step.sum.to_f64());
    }
}
