use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::geo::Vector;

pub fn vector_add(c: &mut Criterion) {
    c.bench_function("vector add", |b| b.iter(|| {
        let _ = Vector::splat(black_box(1.0)) + Vector::splat(black_box(2.0));
    }));
}

pub fn vector_premult_f64(c: &mut Criterion) {
    let v = Vector::splat(2.0);

    c.bench_function("vector pre-mult f64", |b| b.iter(|| {
        let _ = black_box(2.0) * v;
    }));
}

pub fn vector_postmult_f64(c: &mut Criterion) {
    let v = Vector::splat(2.0);

    c.bench_function("vector post-mult f64", |b| b.iter(|| {
        let _ = v * black_box(2.0);
    }));
}

pub fn vector_div_f64(c: &mut Criterion) {
    let v = Vector::splat(2.345);

    c.bench_function("vector div f64", |b| b.iter(|| {
        let _ = v / black_box(2.0);
    }));
}

criterion_group!(geo, 
    vector_add, 
    vector_premult_f64,
    vector_postmult_f64,
    vector_div_f64);
criterion_main!(geo);