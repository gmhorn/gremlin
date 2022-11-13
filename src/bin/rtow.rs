use gremlin::{prelude::*, film::{RGBFilm, RGB}, geo::{Ray, Unit}, camera::Perspective};

const WHITE: RGB = RGB::new(1.0, 1.0, 1.0);
const BLACK: RGB = RGB::new(0.0, 0.0, 0.0);
const BLUE: RGB = RGB::new(0.5, 0.7, 1.0);

fn ray_color(ray: Ray) -> RGB {
    if let Ok(unit) = Unit::try_from(ray.direction()) {
        let t = 0.5*(unit.y() + 1.0);
        WHITE*(1.0-t) + BLUE*t
    } else {
        BLACK
    }
}

fn main() {
    let mut img = RGBFilm::new(800, 600);
    let mut cam = Perspective::new(img.aspect_ratio(), 75.0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // let ray = cam.ray(u, v)
    }
    let scale_x = (img.width() as Float) - 1.0;
    let scale_y = (img.height() as Float) - 1.0;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x as Float) / scale_x;
        let g = (y as Float) / scale_y;
        let b = 0.25;

        pixel.add_sample(RGB::new(r, g, b));
    }
    img.snapshot().save_image("rtow.png").unwrap();
}
