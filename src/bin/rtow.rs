use gremlin::{film::{RGBFilm, RGB, Save}, Float};

fn main() {
    let mut img = RGBFilm::new(800, 600);
    let scale_x = (img.width() as Float)-1.0;
    let scale_y = (img.height() as Float)-1.0;
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x as Float) / scale_x;
        let g = (y as Float) / scale_y;
        let b = 0.25;

        pixel.add_sample(RGB::new(r, g, b));
    }
    img.snapshot().save_image("rtow.png").unwrap();
}
