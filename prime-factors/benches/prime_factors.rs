use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prime_factors::factors;

fn bench_prime_factors(c: &mut Criterion) {
    c.bench_function("factor of 1", |b| b.iter(|| factors(black_box(1))));
    c.bench_function("factor of 2", |b| b.iter(|| factors(black_box(2))));
    c.bench_function("factor of small number", |b| b.iter(|| factors(black_box(901_255))));
    c.bench_function("factor of big number", |b| b.iter(|| factors(black_box(93_819_012_551))));
}

fn other_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidates = 2..;
    while n > 1 {
        let x = candidates.next().unwrap();
        while n % x == 0 {
            n /= x;
            factors.push(x);
        }
    }
    factors
}

fn bench_other_prime_factors(c: &mut Criterion) {
    c.bench_function("factor of 1", |b| b.iter(|| other_factors(black_box(1))));
    c.bench_function("factor of 2", |b| b.iter(|| other_factors(black_box(2))));
    c.bench_function("factor of small number", |b| b.iter(|| other_factors(black_box(901_255))));
    c.bench_function("factor of big number", |b| b.iter(|| other_factors(black_box(93_819_012_551))));
}

criterion_group!(benches, bench_prime_factors, bench_other_prime_factors);
criterion_main!(benches);