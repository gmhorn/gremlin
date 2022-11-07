use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::spectrum::{self, Sampled};

pub fn sampled_splat(c: &mut Criterion) {
    c.bench_function("sampled splat", |b| {
        b.iter(|| {
            let _ = black_box(Sampled::splat(1.0));
        })
    });
}

pub fn sampled_clone(c: &mut Criterion) {
    let spec = Sampled::from(|w| spectrum::blackbody(6500.0, w));
    c.bench_function("sampled clone", |b| {
        b.iter(|| {
            let _ = black_box(spec.clone());
        })
    });
}

pub fn sampled_from_blackbody(c: &mut Criterion) {
    c.bench_function("sampled from blackbody", |b| {
        b.iter(|| {
            let _ = black_box(Sampled::from(|w| spectrum::blackbody(6500.0, w)));
        })
    });
}

criterion_group!(spectrum, sampled_splat, sampled_clone, sampled_from_blackbody);
criterion_main!(spectrum);
