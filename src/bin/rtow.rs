use gremlin::{
    camera::Perspective,
    film::{RGBFilm},
    geo::{Point, Ray},
    metrics::{Counter, Timer},
    prelude::*,
    shape::{Intersection, Sphere, Surface}, color::RGB,
};
use rand::prelude::*;

static RAY_COUNT: Counter = Counter::new();

const WHITE: RGB = RGB::new(1.0, 1.0, 1.0);
const BLUE: RGB = RGB::new(0.3, 0.5, 1.0);

fn ray_color(ray: &Ray, isect: Option<Intersection>) -> RGB {
    if let Some(isect) = isect {
        RGB::new(
            isect.norm.x() + 1.0,
            isect.norm.y() + 1.0,
            isect.norm.z() + 1.0,
        ) * 0.5
    } else {
        let dir = ray.direction().normalize();
        let t = 0.5 * (dir.y() + 1.0);
        WHITE * (1.0 - t) + BLUE * t
    }
}

fn main() {
    let mut img = RGBFilm::new(800, 600);
    let mut cam = Perspective::new(img.aspect_ratio(), 75.0);
    cam.move_to(0.0, 0.0, 1.0);
    cam.look_at(0.0, 0.0, -1.0);

    let sphere = Surface::from(Sphere::new(Point::new(0.0, 0.0, -1.0), 1.0));

    let timer = Timer::tick();
    for _ in 0..50 {
        img.add_samples(|x, y| {
            let mut rng = rand::thread_rng();
            RAY_COUNT.inc();
            let ray = cam.ray(x + rng.gen::<Float>(), y + rng.gen::<Float>());
            let isect = sphere.intersect(&ray, 0.0, Float::INFINITY);
            ray_color(&ray, isect)
        });
    }
    println!("Traced {} rays in {:?}", RAY_COUNT.get(), timer.tock());
    println!(
        "{} Rays/Sec",
        RAY_COUNT.get() as f64 / timer.tock().as_secs_f64()
    );

    img.to_snapshot().save_image("rtow.png").unwrap();
}
