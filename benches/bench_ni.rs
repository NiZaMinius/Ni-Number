//! Criterion benchmarks for the ni-number crate.
//!
//! Run with: `cargo bench`

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ni_number::{ni_number, bits_for_digits};

fn bench_ni_precision(c: &mut Criterion) {
    let mut group = c.benchmark_group("ni_number");

    for digits in [50u32, 100, 500, 1000, 5000] {
        let bits = bits_for_digits(digits);
        group.bench_with_input(
            BenchmarkId::new("digits", digits),
            &bits,
            |b, &bits| {
                b.iter(|| ni_number(black_box(bits)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_ni_precision);
criterion_main!(benches);
