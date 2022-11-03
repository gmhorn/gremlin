use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::spectrum::Sampled64;

pub fn spectrum_add_iter(c: &mut Criterion) {
    let s1 = Sampled64::splat(1.0);
    let s2 = Sampled64::splat(2.0);

    c.bench_function("spectrum add iter", |b| {
        b.iter(|| {
            let _ = black_box(s1.add_iter(&s2));
        })
    });
}

criterion_group!(spectrum, spectrum_add_iter,);
criterion_main!(spectrum);
