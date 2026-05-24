//! High-precision computation example for the `ni-number` crate.
//!
//! Demonstrates computing thousands of decimal digits efficiently,
//! even on low-end hardware, by exploiting the fast convergence of η_ν.
//!
//! Run with: `cargo run --example high_precision --release`
use ni_number::{clear_cache, ni_number_digits};
use std::time::Instant;

fn main() {
    println!("═══════════════════════════════════════════════════");
    println!("  η_ν High-Precision Computation Benchmark");
    println!("═══════════════════════════════════════════════════\n");
    let digits = 10_000;

    // We clear the cache just in case to measure the cold start
    clear_cache();

    let start = Instant::now();
    let result = ni_number_digits(digits);
    let duration = start.elapsed();

    // We only print the beginning and end of the line to avoid cluttering the terminal.
    let prefix = &result[..50];
    let suffix = &result[result.len() - 50..];

    println!("Computed {} decimal digits in {:?}", digits, duration);
    println!("Result: {} ... {}", prefix, suffix);
}
