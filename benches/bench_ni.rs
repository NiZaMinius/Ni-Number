//! Criterion benchmarks for the ni-number crate.
//!
//! Run with: `cargo bench`
//! Run with rug: `cargo bench --features backend-rug`

use criterion::{BatchSize, BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use ni_number::compute;

// ─── Dashu backend ────────────────────────────────────────────────────────────

#[cfg(feature = "backend-dashu")]
fn bench_dashu(c: &mut Criterion) {
    use ni_number::backend::dashu::DashuBackend;

    let mut group = c.benchmark_group("dashu/cold");
    group.sample_size(10);

    for digits in [100u32, 1_000, 5_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(digits), digits, |b, &d| {
            b.iter_batched(
                // Setup: clear π cache so each iteration is a cold start
                || ni_number::clear_cache(),
                |_| black_box(compute::compute::<DashuBackend>(compute::digits_to_bits(d))),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();

    // Warm path: π is already cached, only the series is computed
    let mut group = c.benchmark_group("dashu/warm");
    group.sample_size(20);

    for digits in [100u32, 1_000, 5_000].iter() {
        // Pre-warm the π cache
        let _ = compute::compute::<DashuBackend>(compute::digits_to_bits(*digits));

        group.bench_with_input(BenchmarkId::from_parameter(digits), digits, |b, &d| {
            b.iter_batched(
                || ni_number::clear_cache(),
                |_| black_box(compute::compute::<DashuBackend>(compute::digits_to_bits(d))),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

// ─── Rug backend ──────────────────────────────────────────────────────────────

#[cfg(feature = "backend-rug")]
fn bench_rug(c: &mut Criterion) {
    use ni_number::backend::rug::RugBackend;

    let mut group = c.benchmark_group("rug/cold");
    group.sample_size(10);

    for digits in [100u32, 1_000, 5_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(digits), digits, |b, &d| {
            b.iter_batched(
                || (),
                |_| black_box(compute::compute::<RugBackend>(compute::digits_to_bits(d))),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

// ─── Entry points ─────────────────────────────────────────────────────────────

#[cfg(feature = "backend-dashu")]
criterion_group!(benches_dashu, bench_dashu);

#[cfg(feature = "backend-rug")]
criterion_group!(benches_rug, bench_rug);

// criterion_main! requires all groups to exist at compile time,
// so we conditionally compile the main entry point.
#[cfg(all(feature = "backend-dashu", feature = "backend-rug"))]
criterion_main!(benches_dashu, benches_rug);

#[cfg(all(feature = "backend-dashu", not(feature = "backend-rug")))]
criterion_main!(benches_dashu);

#[cfg(all(feature = "backend-rug", not(feature = "backend-dashu")))]
criterion_main!(benches_rug);
