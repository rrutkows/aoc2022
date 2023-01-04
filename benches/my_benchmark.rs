use aoc2022::{d14, d15, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25};
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
    c.bench_function("d18p1", |b| b.iter(d18::solve01));
    c.bench_function("d18p2", |b| b.iter(d18::solve02));
    c.bench_function("d19p1", |b| b.iter(d19::solve01));
    c.bench_function("d19p2", |b| b.iter(d19::solve02));
    c.bench_function("d20p1", |b| b.iter(|| d20::solve(1, 1)));
    c.bench_function("d20p2", |b| b.iter(|| d20::solve(811589153, 10)));
    c.bench_function("d21p1", |b| b.iter(d21::solve01));
    c.bench_function("d21p2", |b| b.iter(d21::solve02));
    c.bench_function("d22p1", |b| b.iter(d22::solve01));
    c.bench_function("d22p2", |b| b.iter(d22::solve02));
    c.bench_function("d23p1", |b| b.iter(d23::solve01));
    c.bench_function("d23p2", |b| b.iter(d23::solve02));
    c.bench_function("d24p1", |b| b.iter(d24::solve01));
    c.bench_function("d24p2", |b| b.iter(d24::solve02));
    c.bench_function("d25p1", |b| b.iter(d25::solve01));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
