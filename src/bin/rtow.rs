use gremlin::{
    camera::Perspective,
    film::{RGBFilm, RGB},
    geo::{Ray, Unit},
    prelude::*,
};
use rayon::prelude::*;

const WHITE: RGB = RGB::new(1.0, 1.0, 1.0);
const BLACK: RGB = RGB::new(0.0, 0.0, 0.0);
const BLUE: RGB = RGB::new(0.3, 0.5, 1.0);

fn ray_color(ray: Ray) -> RGB {
    if let Ok(unit) = Unit::try_from(ray.direction()) {
        let t = 0.5 * (unit.y() + 1.0);
        WHITE * (1.0 - t) + BLUE * t
    } else {
        BLACK
    }
}

fn main() {
    let mut img = RGBFilm::new(800, 600);
    let cam = Perspective::new(img.aspect_ratio(), 75.0);

    img.enumerate_ndc_mut()
        .par_bridge()
        .for_each(|(u, v, pixel)| {
            pixel.add_sample(ray_color(cam.ray(u, v)));
        });
    img.snapshot().save_image("rtow.png").unwrap();
}
