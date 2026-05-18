//! High-precision computation example for the `ni-number` crate.
//!
//! Demonstrates computing thousands of decimal digits efficiently,
//! even on low-end hardware, by exploiting the fast convergence of η_ν.
//!
//! Run with: `cargo run --example high_precision --release`

use ni_number::{ni_number_digits, bits_for_digits, ni_number};
use std::time::Instant;

fn main() {
    println!("═══════════════════════════════════════════════════");
    println!("  η_ν High-Precision Computation Benchmark");
    println!("═══════════════════════════════════════════════════\n");

    let targets: &[u32] = &[100, 500, 1_000, 5_000, 10_000];

    for &digits in targets {
        let t0 = Instant::now();
        let s = ni_number_digits(digits);
        let elapsed = t0.elapsed();

        // Show first 40 and last 10 chars to verify
        let preview = if s.len() > 55 {
            format!("{}...{}", &s[..45], &s[s.len()-10..])
        } else {
            s.clone()
        };

        println!(
            "  {:>6} digits in {:>8.3?} ms  →  {}",
            digits,
            elapsed.as_secs_f64() * 1000.0,
            preview
        );
    }

    println!("\n═══════════════════════════════════════════════════");
    println!("  Full 1000-digit output:");
    println!("═══════════════════════════════════════════════════\n");

    let full = ni_number(bits_for_digits(1000));
    // Print in groups of 10 digits, 5 groups per line
    let s = format!("{:.1000}", full);
    // Remove "1." prefix, print digits in chunks
    if let Some(dot_pos) = s.find('.') {
        let integer_part = &s[..dot_pos];
        let frac_part = &s[dot_pos+1..];
        print!("η_ν = {}.", integer_part);
        for (i, chunk) in frac_part.chars().collect::<Vec<_>>().chunks(10).enumerate() {
            if i > 0 && i % 5 == 0 {
                println!();
                print!("        ");
            }
            print!("{} ", chunk.iter().collect::<String>());
        }
        println!();
    }
}
