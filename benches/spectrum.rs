use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::spectrum::{Sampled, Peak, self};

pub fn spectrum_integrate(c: &mut Criterion) {
    let f = Sampled::splat(1.0);
    let g = Sampled::from(Peak::new(550.0, 15.0));

    c.bench_function("riemann integrate", |b| {
        b.iter(|| {
            let _ = black_box(spectrum::integrate(&f, &g));
        })
    });
}

pub fn spectrum_integrate_3(c: &mut Criterion) {
    let f = Sampled::splat(1.0);
    let g = Sampled::from(Peak::new(550.0, 15.0));

    c.bench_function("riemann integrate x3", |b| {
        b.iter(|| {
            let _ = black_box(spectrum::integrate_3(&f, &g));
        })
    });
}

criterion_group!(spectrum, spectrum_integrate, spectrum_integrate_3,);
criterion_main!(spectrum);
