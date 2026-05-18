//! Basic usage example for the `ni-number` crate.
//!
//! Run with: `cargo run --example basic`

use ni_number::{NI_F64, ni_number_digits, ni_term, bits_for_digits};

fn main() {
    println!("══════════════════════════════════════════════");
    println!("  Ni Constant (η_ν) — ni-number crate demo");
    println!("══════════════════════════════════════════════\n");

    // 1. Quick f64 constant
    println!("► f64 constant (fast, ~15 digits):");
    println!("  η_ν ≈ {:.15}\n", NI_F64);

    // 2. 50 decimal digits
    println!("► 50 decimal digits:");
    let d50 = ni_number_digits(50);
    println!("  η_ν = {}\n", d50);

    // 3. 100 decimal digits
    println!("► 100 decimal digits:");
    let d100 = ni_number_digits(100);
    println!("  η_ν = {}\n", d100);

    // 4. Show individual terms of the series
    println!("► Individual series terms  πⁿ / (n! · 2^(n²)):");
    let bits = bits_for_digits(30);
    for n in 1..=10 {
        let term = ni_term(n, bits);
        println!("  term({:2}) = {:.20e}", n, term);
    }

    println!("\n► Convergence check: running sum");
    let mut sum = rug::Float::with_val(bits, 0.0);
    for n in 1..=15 {
        sum += ni_term(n, bits);
        println!("  after n={:2}: {:.15}", n, sum);
    }
}
