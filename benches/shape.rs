use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gremlin::{
    geo::{Matrix, Point, Ray, Vector},
    shape::*,
    Float,
};
use rand::prelude::*;

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

pub fn aggregate_direct_dispatch(c: &mut Criterion) {
    let agg = random_spheres();
    let ray = Ray::new(Point::new(0.0, 0.0, -20.0), Vector::Z_AXIS);

    c.bench_function("aggregate direct dispatch", |b| {
        b.iter(|| {
            let _ = black_box(agg.intersect(&ray, 0.0, Float::INFINITY));
        })
    });
}

pub fn aggregate_enum_dispatch(c: &mut Criterion) {
    let agg: Vec<Surface> = random_spheres()
        .into_iter()
        .map(|sphere| Surface::from(sphere))
        .collect();
    let ray = Ray::new(Point::new(0.0, 0.0, -20.0), Vector::Z_AXIS);

    c.bench_function("aggregate enum dispatch", |b| {
        b.iter(|| {
            let _ = black_box(agg.intersect(&ray, 0.0, Float::INFINITY));
        })
    });
}

pub fn aggregate_dynamic_dispatch(c: &mut Criterion) {
    let mut agg = DynamicAggregate::new();
    for sphere in random_spheres() {
        agg.push(Box::new(sphere));
    }
    let ray = Ray::new(Point::new(0.0, 0.0, -20.0), Vector::Z_AXIS);

    c.bench_function("aggregate dynamic dispatch", |b| {
        b.iter(|| {
            let _ = black_box(agg.intersect(&ray, 0.0, Float::INFINITY));
        })
    });
}

fn random_spheres() -> Vec<Sphere> {
    let mut rng = StdRng::seed_from_u64(1234);
    let m = Matrix::scale_uniform(10.0);
    (0..1024)
        .into_iter()
        .map(|_| {
            let p = Point::new(rng.gen(), rng.gen(), rng.gen());
            Sphere::new(m * p, rng.gen())
        })
        .collect()
}

criterion_group!(
    shape,
    direct_dispatch,
    enum_dispatch,
    dynamic_dispatch,
    aggregate_direct_dispatch,
    aggregate_enum_dispatch,
    aggregate_dynamic_dispatch,
);
criterion_main!(shape);
