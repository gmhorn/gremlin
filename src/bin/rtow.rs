use std::time::Instant;

use gremlin::{
    camera::Perspective,
    film::{RGBFilm, RGB},
    geo::{Point, Ray},
    prelude::*,
    shape::{Intersection, Sphere},
};
use rayon::prelude::*;

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

    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 1.0);

    let start = Instant::now();
    for _ in 0..1 {
        img.enumerate_ndc_mut()
            .par_bridge()
            .for_each(|(u, v, pixel)| {
                let ray = cam.ray(u, v);
                let isect = sphere.intersect(&ray, 0.0, Float::INFINITY);
                pixel.add_sample(ray_color(&ray, isect));
            });
    }
    let dur = Instant::now() - start;
    println!("Took {:?}", dur);

    img.snapshot().save_image("rtow.png").unwrap();
}
