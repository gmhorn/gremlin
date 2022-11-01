use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::geo::{Matrix, Vector};

pub fn vector_min(c: &mut Criterion) {
    let u = Vector::splat(1.0);
    let v = Vector::new(1.0, 2.0, 3.0);

    c.bench_function("vector min", |b| {
        b.iter(|| {
            let _ = black_box(Vector::min(u, v));
        })
    });
}

pub fn vector_add(c: &mut Criterion) {
    let u = Vector::splat(1.0);
    let v = Vector::new(1.0, 2.0, 3.0);

    c.bench_function("vector add", |b| {
        b.iter(|| {
            let _ = black_box(u + v);
        })
    });
}

pub fn vector_scalar_mult(c: &mut Criterion) {
    let v = Vector::splat(2.0);

    c.bench_function("vector scalar mult", |b| {
        b.iter(|| {
            let _ = black_box(v * 2.0);
        })
    });
}

pub fn vector_scalar_div(c: &mut Criterion) {
    let v = Vector::splat(2.345);

    c.bench_function("vector scalar div", |b| {
        b.iter(|| {
            let _ = black_box(v / 2.0);
        })
    });
}

pub fn matrix_add(c: &mut Criterion) {
    let m = Matrix::scale_uniform(1.0);
    let n = Matrix::scale_uniform(3.6);

    c.bench_function("matrix add", |b| {
        b.iter(|| {
            let _ = black_box(m + n);
        })
    });
}

pub fn matrix_scalar_mult(c: &mut Criterion) {
    let m = Matrix::scale_uniform(3.6);

    c.bench_function("matrix scalar mult", |b| {
        b.iter(|| {
            let _ = black_box(m * 2.0);
        });
    });
}

pub fn matrix_vector_mult(c: &mut Criterion) {
    let m = Matrix::rotate(50.0, Vector::new(1.0, 2.0, 3.0).normalize());
    let v = Vector::new(1.0, 2.0, 3.0);

    c.bench_function("matrix vector mult", |b| {
        b.iter(|| {
            let _ = black_box(m * v);
        });
    });
}

criterion_group!(
    geo,
    vector_min,
    vector_add,
    vector_scalar_mult,
    vector_scalar_div,
    matrix_add,
    matrix_scalar_mult,
    matrix_vector_mult,
);
criterion_main!(geo);
