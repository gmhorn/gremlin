use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::{
    geo::{Point, Ray, Vector},
    shape::*,
    Float,
};

pub fn direct_dispatch(c: &mut Criterion) {
    let sphere = Sphere::new(Point::new(10.0, 0.0, 0.0), 1.0);
    let ray = Ray::new(Point::ORIGIN, Vector::X_AXIS);

    c.bench_function("direct dispatch", |b| {
        b.iter(|| {
            let _ = black_box(sphere.intersect(&ray, 0.0, Float::INFINITY));
        })
    });
}

pub fn enum_dispatch(c: &mut Criterion) {
    let surf = Surface::from(Sphere::new(Point::new(10.0, 0.0, 0.0), 1.0));
    let ray = Ray::new(Point::ORIGIN, Vector::X_AXIS);

    c.bench_function("enum dispatch", |b| {
        b.iter(|| {
            let _ = black_box(surf.intersect(&ray, 0.0, Float::INFINITY));
        })
    });
}

pub fn dynamic_dispatch(c: &mut Criterion) {
    let shape: Box<dyn Shape> = Box::new(Sphere::new(Point::new(10.0, 0.0, 0.0), 1.0));
    let ray = Ray::new(Point::ORIGIN, Vector::X_AXIS);

    c.bench_function("dynamic dispatch", |b| {
        b.iter(|| {
            let _ = black_box(shape.intersect(&ray, 0.0, Float::INFINITY));
        })
    });
}

criterion_group!(shape, direct_dispatch, enum_dispatch, dynamic_dispatch,);
criterion_main!(shape);
