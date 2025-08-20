use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use swar_simd::{swar, sse};

const MB: usize = 0x100000;

static BYTES: [u8; 2 * MB] = {
    let mut a = [0; 2 * MB];
    *a.last_mut().unwrap() = 69;
    a
};

fn iter(value: &[u8], byte: u8) -> Option<usize> {
    value.iter().position(|e| e == &byte)
}

// https://bheisler.github.io/criterion.rs/book/user_guide/comparing_functions.html
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find");

    group.bench_function("find iter", |b| {
        b.iter(|| iter(black_box(&BYTES), black_box(69)).unwrap())
    });

    group.bench_function("find swar", |b| {
        b.iter(|| swar::find(black_box(&BYTES), black_box(69)).unwrap())
    });

    group.bench_function("find sse", |b| {
        b.iter(|| sse::find(black_box(&BYTES), black_box(69)).unwrap())
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
