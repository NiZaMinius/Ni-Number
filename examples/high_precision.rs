//! High-precision computation example for the `ni-number` crate.
//!
//! Demonstrates computing thousands of decimal digits efficiently,
//! even on low-end hardware, by exploiting the fast convergence of η_ν.
//!
//! Run with:
//!   cargo run --example high_precision --release
//!   cargo run --example high_precision --release --features backend-rug

use ni_number::backend::NiFloat;
use ni_number::{bits_for_digits, clear_cache, ni_number};
use std::time::Instant;

fn main() {
    println!("═══════════════════════════════════════════════════");
    println!("  η_ν High-Precision Computation Benchmark");
    println!("═══════════════════════════════════════════════════\n");

    let digits = 10_000u32;

    // ── Dashu (default backend) ───────────────────────────────────────────────
    #[cfg(feature = "backend-dashu")]
    {
        use ni_number::backend::dashu::DashuBackend;
        use ni_number::compute;

        clear_cache();

        let start = Instant::now();
        let val = compute::compute::<DashuBackend>(compute::digits_to_bits(digits));
        let compute_time = start.elapsed();

        let start = Instant::now();
        let result = val.to_decimal_string(digits);
        let format_time = start.elapsed();

        let prefix = &result[..50];
        let suffix = &result[result.len() - 50..];

        println!("Dashu backend ({} digits):", digits);
        println!("  Series computation : {:?}", compute_time);
        println!("  Base-2 → Base-10   : {:?}", format_time);
        println!("  Total              : {:?}", compute_time + format_time);
        println!("  Result: {} ... {}", prefix, suffix);
        println!();
        println!("  Note: for 10 000+ digits, formatting dominates.");
        println!("        Use --features backend-rug for practical performance.");
    }

    // ── Rug backend (GNU MPFR) ────────────────────────────────────────────────
    #[cfg(feature = "backend-rug")]
    {
        use ni_number::backend::rug::RugBackend;
        use ni_number::compute;

        clear_cache();

        let start = Instant::now();
        let val = compute::compute::<RugBackend>(compute::digits_to_bits(digits));
        let compute_time = start.elapsed();

        let start = Instant::now();
        let result = val.to_decimal_string(digits);
        let format_time = start.elapsed();

        let prefix = &result[..50];
        let suffix = &result[result.len() - 50..];

        println!("Rug backend ({} digits):", digits);
        println!("  Series computation : {:?}", compute_time);
        println!("  Formatting         : {:?}", format_time);
        println!("  Total              : {:?}", compute_time + format_time);
        println!("  Result: {} ... {}", prefix, suffix);
    }
}
