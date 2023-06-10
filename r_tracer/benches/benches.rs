use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render_suzanne_after_bvh_update", |b| {
        b.iter_batched(
            || 20,
            |num_samples| r_tracer::render_suzanne(black_box(num_samples)),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
