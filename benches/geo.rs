use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::geo::{Matrix, Vec3, Vector, Mtx4};

pub fn vector_min(c: &mut Criterion) {
    c.bench_function("vector min", |b| {
        b.iter(|| {
            let _ = Vector::min(
                Vector::splat(black_box(1.0)),
                Vector::new(black_box(-1.0), black_box(2.0), black_box(1.0)),
            );
        })
    });
}

pub fn vector_add(c: &mut Criterion) {
    c.bench_function("vector add", |b| {
        b.iter(|| {
            let _ = Vector::splat(black_box(1.0)) + Vector::splat(black_box(2.0));
        })
    });
}

// pub fn vector_unit_add(c: &mut Criterion) {
//     c.bench_function("vector unit add", |b| {
//         b.iter(|| {
//             let _ = Vector::splat(black_box(1.0)) + Unit::X_AXIS;
//         })
//     });
// }

pub fn vector_premult_f64(c: &mut Criterion) {
    let v = Vector::splat(2.0);

    c.bench_function("vector pre-mult f64", |b| {
        b.iter(|| {
            let _ = black_box(2.0) * v;
        })
    });
}

pub fn vector_postmult_f64(c: &mut Criterion) {
    let v = Vector::splat(2.0);

    c.bench_function("vector post-mult f64", |b| {
        b.iter(|| {
            let _ = v * black_box(2.0);
        })
    });
}

pub fn vector_div_f64(c: &mut Criterion) {
    let v = Vector::splat(2.345);

    c.bench_function("vector div f64", |b| {
        b.iter(|| {
            let _ = v / black_box(2.0);
        })
    });
}

pub fn matrix_vector_mult(c: &mut Criterion) {
    let m = Matrix::rotate(50.0, Vector::new(1.0, 2.0, 3.0).normalize());

    c.bench_function("matrix vector mult", |b| {
        b.iter(|| {
            let _ = &m * Vector::new(black_box(1.0), black_box(2.0), black_box(3.0));
        });
    });
}

pub fn matrix_add(c: &mut Criterion) {
    let m = Mtx4::scale(1.0);
    let n = Mtx4::scale(3.6);

    c.bench_function("matrix add", |b| {
        b.iter(|| {
            let _ = black_box(m + n);
        })
    });
}

criterion_group!(
    geo,
    // vector_min,
    // vector_add,
    // vector_unit_add,
    // vector_premult_f64,
    // vector_postmult_f64,
    // vector_div_f64,
    matrix_add
    // matrix_vector_mult
);
criterion_main!(geo);
