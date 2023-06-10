use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render_suzanne_after_optimization", |b| b.iter(|| r_tracer::render_suzanne(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);