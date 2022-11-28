use gremlin::{
    camera::ThinLens,
    color::RGB,
    film::RGBFilm,
    geo::{Point, Ray, Vector},
    metrics::{Counter, Timer},
    prelude::*,
    shape::{Sphere, Surface},
};
use rand::prelude::*;
use rand_distr::UnitSphere;
use rayon::prelude::*;

static RAY_COUNT: Counter = Counter::new();

const WHITE: [Float; 3] = [1.0, 1.0, 1.0];
const BLUE: [Float; 3] = [0.3, 0.5, 1.0];
const BLACK: [Float; 3] = [0.0, 0.0, 0.0];

fn ray_color(ray: Ray, surfaces: &impl Shape, depth: usize, rng: &mut impl Rng) -> RGB {
    RAY_COUNT.inc();

    if let Some(isect) = surfaces.intersect(&ray, 0.001, Float::INFINITY) {
        if depth < 50 {
            let rand_vec = Vector::from(UnitSphere.sample(rng));
            let target = isect.point + isect.norm.into() + rand_vec;
            let ray = Ray::new(isect.point, target - isect.point);
            ray_color(ray, surfaces, depth + 1, rng) * 0.5
        } else {
            RGB::from(BLACK)
        }
    } else {
        let dir = ray.direction().normalize();
        let t = 0.5 * (dir.y() + 1.0);
        RGB::from(WHITE) * (1.0 - t) + RGB::from(BLUE) * t
    }
}

fn main() {
    let mut img = RGBFilm::new(800, 600);
    let cam = ThinLens::builder(img.dimensions())
        .move_to([1.0, 0.5, 1.0])
        .look_at([0.0, 0.0, -1.0])
        .fov(55.0)
        .aperture(0.25)
        .auto_focus()
        .build();

    let surfaces: Vec<Surface> = vec![
        Surface::from(Sphere::new(Point::new(-0.5, 0.0, -1.0), 0.5)),
        Surface::from(Sphere::new(Point::new(-0.5, 0.0, -2.0), 0.5)),
        Surface::from(Sphere::new(Point::new(0.5, 0.0, -1.0), 0.5)),
        Surface::from(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let timer = Timer::tick();
    for _ in 0..128 {
        img.par_pixel_iter_mut()
            .for_each_init(rand::thread_rng, |rng, (px, py, pixel)| {
                let ray = cam.ray(px, py, rng);
                pixel.add_sample(ray_color(ray, &surfaces, 0, rng));
            });
    }

    println!("Traced {} rays in {:?}", RAY_COUNT.get(), timer.tock());
    println!(
        "{} Rays/Sec",
        RAY_COUNT.get() as f64 / timer.tock().as_secs_f64()
    );

    img.to_snapshot().save_image("rtow-thinlens.png").unwrap();
}
