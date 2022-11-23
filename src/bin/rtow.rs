use gremlin::{
    camera::Perspective,
    color::RGB,
    film::RGBFilm,
    geo::{Point, Ray, Vector},
    metrics::{Counter, Timer},
    prelude::*,
    shape::{Intersection, Sphere, Surface},
};
use rand::prelude::*;
use rand_distr::UnitSphere;

static RAY_COUNT: Counter = Counter::new();

const WHITE: [Float; 3] = [1.0, 1.0, 1.0];
const BLUE: [Float; 3] = [0.3, 0.5, 1.0];
const BLACK: [Float; 3] = [0.0, 0.0, 0.0];

fn ray_color(ray: &Ray, isect: Option<Intersection>) -> RGB {
    if let Some(isect) = isect {
        RGB::from([
            isect.norm.x() + 1.0,
            isect.norm.y() + 1.0,
            isect.norm.z() + 1.0,
        ]) * 0.5
    } else {
        let dir = ray.direction().normalize();
        let t = 0.5 * (dir.y() + 1.0);
        RGB::from(WHITE) * (1.0 - t) + RGB::from(BLUE) * t
    }
}

fn ray_color_2(ray: Ray, surfaces: &impl Shape, depth: usize, rng: &mut impl Rng) -> RGB {
    RAY_COUNT.inc();

    if let Some(isect) = surfaces.intersect(&ray, 0.001, Float::INFINITY) {
        if depth < 50 {
            let rand_vec = Vector::from(UnitSphere.sample(rng));
            let target = isect.point + isect.norm.into() + rand_vec;
            let ray = Ray::new(isect.point, target - isect.point);
            ray_color_2(ray, surfaces, depth + 1, rng) * 0.5
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
    
    let mut cam = Perspective::new(img.aspect_ratio(), 55.0);
    cam.move_to([1.0, 0.5, 1.5]);
    // cam.move_to(-3.0, 3.0, 1.0);
    // cam.look_at(0.0, 0.0, -1.0);

    let mut surfaces: Vec<Surface> = vec![];
    surfaces.push(Surface::from(Sphere::new(Point::new(-0.5, 0.0, -1.0), 0.5)));
    surfaces.push(Surface::from(Sphere::new(Point::new(-0.5, 0.0, -2.0), 0.5)));
    surfaces.push(Surface::from(Sphere::new(Point::new(0.5, 0.0, -1.0), 0.5)));
    surfaces.push(Surface::from(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let raster_to_ndc = img.raster_to_ndc();
    let timer = Timer::tick();
    for _ in 0..1024 {
        img.add_samples(|x, y| {
            let mut rng = rand::thread_rng();
            let (u, v) = raster_to_ndc(x + rng.gen::<Float>(), y + rng.gen::<Float>());

            ray_color_2(cam.ray(u, v), &surfaces, 0, &mut rng)
        });
    }
    println!("Traced {} rays in {:?}", RAY_COUNT.get(), timer.tock());
    println!(
        "{} Rays/Sec",
        RAY_COUNT.get() as f64 / timer.tock().as_secs_f64()
    );

    img.to_snapshot().save_image("rtow.png").unwrap();
}
