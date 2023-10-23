use criterion::{black_box, criterion_group, criterion_main, Criterion };
use nth_prime::*;

fn bench_nth_prime(c: &mut Criterion) {
    c.bench_function("nth less than 10", |b| b.iter(|| black_box(nth(10))));
    c.bench_function("nth bigger than 10 less than 1000", |b| b.iter(|| black_box(nth(1000))));
    c.bench_function("nth big", |b| b.iter(|| black_box(nth(10000))));
}

/// briankung's solution
fn nth_prime(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|candidate: &u32| {
            if !primes.iter().any(|i| candidate % i == 0) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}

fn bench_nth_prime_brain(c: &mut Criterion) {
    c.bench_function("nth less than 10", |b| b.iter(|| black_box(nth_prime(10))));
    c.bench_function("nth bigger than 10 less than 1000", |b| b.iter(|| black_box(nth_prime(1000))));
    c.bench_function("nth big", |b| b.iter(|| black_box(nth_prime(10000))));
}

criterion_group!(benches, bench_nth_prime, bench_nth_prime_brain);
criterion_main!(benches);