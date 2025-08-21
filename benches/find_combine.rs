use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use swar_simd::swar;

const MB: usize = 0x100000;

static BYTES: [u8; 2 * MB] = {
    let mut a = [b'!'; 2 * MB];
    *a.last_mut().unwrap() = b'\n';
    a
};

fn iter(bytes: &[u8]) -> Option<usize> {
    bytes
        .iter()
        .position(|byte| matches!(byte, b'\n' | b'\r') || !matches!(byte, b'!'..=b'~'))
}

fn find_combine(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find Combine");

    group.bench_function("find_combine iter", |b| {
        b.iter(|| iter(black_box(&BYTES)).unwrap())
    });

    group.bench_function("find_combine scalar", |b| {
        b.iter(|| swar::find_combine_scalar(black_box(&BYTES)).unwrap())
    });

    group.bench_function("find_combine swar", |b| {
        b.iter(|| swar::find_combine(black_box(&BYTES)).unwrap())
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().without_plots().sample_size(50);
    targets = find_combine
);
criterion_main!(benches);
