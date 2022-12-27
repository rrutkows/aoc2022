use aoc2022::{d14, d15, d16, d17, d18};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("d14p1", |b| b.iter(d14::solve01));
    c.bench_function("d14p2", |b| b.iter(d14::solve02));
    c.bench_function("d15p1", |b| b.iter(d15::solve01));
    c.bench_function("d15p2", |b| b.iter(d15::solve02));
    c.bench_function("d16p1", |b| b.iter(|| d16::solve(1)));
    c.bench_function("d16p2", |b| b.iter(|| d16::solve(2)));
    c.bench_function("d17p1", |b| b.iter(|| d17::solve(2022)));
    c.bench_function("d17p2", |b| b.iter(|| d17::solve(1000000000000)));
    c.bench_function("d18p1", |b| b.iter(d18::solve));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
