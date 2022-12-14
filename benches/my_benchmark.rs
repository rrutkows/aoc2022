use aoc2022::d14;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d14p1", |b| b.iter(d14::solve01));
    c.bench_function("d14p2", |b| b.iter(d14::solve02));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
