// benches/benchmark.rs

use criterion::{criterion_group, criterion_main, Criterion};
use custom_create::{two_crystal_balls, two_crystal_balls_my, two_crystal_balls_my_mid, two_cystal_balls_esteban};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut arr_bools = vec![false; 1_000_000];
    arr_bools.iter_mut().skip(800_000).for_each(|val| *val = true);

    let mut group = c.benchmark_group("TwoCrystalBalls");

    group.bench_function("two_crystal_balls", |b| {
        b.iter(|| two_crystal_balls(&arr_bools))
    });

    group.bench_function("two_crystal_balls_my", |b| {
        b.iter(|| two_crystal_balls_my(&arr_bools))
    });

    group.bench_function("two_crystal_balls_esteban", |b| {
        b.iter(|| two_cystal_balls_esteban(&arr_bools))
    });

    group.bench_function("two_crystal_balls_juan_mid", |b| {
        b.iter(|| two_crystal_balls_my_mid(&arr_bools))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

