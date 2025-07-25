use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use swar_simd::find;

const BYTES: [u8; 8192] = [0; 8192];

fn iter(value: &[u8], byte: u8) -> Option<usize> {
    value.iter().position(|e| e == &byte)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find");

    group.bench_function("find iter", |b| {
        b.iter(|| iter(black_box(&BYTES), black_box(69)))
    });

    group.bench_function("find swar", |b| {
        b.iter(|| find(black_box(&BYTES), black_box(69)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
