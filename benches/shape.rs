use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::shape::*;
use rand::prelude::*;

pub fn enum_dispatch(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut shapes: Vec<Surface> = vec![];
    for _ in 0..100000 {
        if rng.gen() {
            shapes.push(Surface::from(Sphere));
        } else {
            shapes.push(Surface::from(Triangle));
        }
    }
    c.bench_function("enum dispatch", |b| {
        b.iter(|| {
            let _: Vec<_> = black_box(shapes.iter().map(|s| s.value()).collect());
        })
    });
}

pub fn dynamic_dispatch(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut shapes: Vec<Box<dyn Shape>> = vec![];
    for _ in 0..100000 {
        if rng.gen() {
            shapes.push(Box::new(Sphere));
        } else {
            shapes.push(Box::new(Triangle));
        }
    }
    c.bench_function("dynamic dispatch", |b| {
        b.iter(|| {
            let _: Vec<_> = black_box(shapes.iter().map(|s| s.value()).collect());
        })
    });
}

criterion_group!(shape, enum_dispatch, dynamic_dispatch,);
criterion_main!(shape);
