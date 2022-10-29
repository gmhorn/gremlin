use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::spectrum::FloatSample;

pub fn spectrum_add_iter(c: &mut Criterion) {
    let s1 = FloatSample::splat(1.0);
    let s2 = FloatSample::splat(2.0);

    c.bench_function("spectrum add iter", |b| {
        b.iter(|| {
            let _ = black_box(s1.add_iter(&s2));
        })
    });
}

criterion_group!(
    spectrum,
    spectrum_add_iter,
);
criterion_main!(spectrum);