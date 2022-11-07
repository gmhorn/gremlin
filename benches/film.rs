use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::{
    film::XYZ,
    spectrum::{self, Sampled},
};

pub fn sampled_to_xyz(c: &mut Criterion) {
    let spec = Sampled::from(|w| spectrum::blackbody(6500.0, w));
    c.bench_function("sampled to XYZ", |b| {
        b.iter(|| {
            let _ = black_box(XYZ::from(spec.clone()));
        })
    });
}

criterion_group!(film, sampled_to_xyz);
criterion_main!(film);
