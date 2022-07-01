#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn new_constructor(c: &mut Criterion) {
    c.bench_function("matrix 2x2 literal data", |b| {
        b.iter(|| {
            black_box(qmat::mat::Matrix::<i32, 2, 2, 4>::new([0, 1, 2, 3]).unwrap());
        })
    });
}

fn matrix_macro_flat(c: &mut Criterion) {
    c.bench_function("matrix 2x2 using matrix!(rows, cols, flat_data)", |b| {
        b.iter(|| {
            black_box(qmat::matrix!(2, 2, [0, 1, 2, 3]));
        })
    });
}

fn matrix_macro_stacked(c: &mut Criterion) {
    c.bench_function("matrix 2x2 using matrix!([[0, 1], [2, 3]])", |b| {
        b.iter(|| {
            black_box(qmat::matrix!([[0, 1], [2, 3]]));
        })
    });
}

criterion_group!(
    benches,
    new_constructor,
    matrix_macro_flat,
    matrix_macro_stacked
);
criterion_main!(benches);
