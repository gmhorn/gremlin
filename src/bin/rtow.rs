use image::RgbImage;

fn main() {
    let imgx = 800;
    let imgy = 600;

    let scalex = (imgx as f32) - 1.0;
    let scaley = (imgy as f32) - 1.0;

    let mut img = RgbImage::new(imgx, imgy);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = ((x as f32) / scalex) * 255.0;
        let g = ((y as f32) / scaley) * 255.0;
        let b = 0.25 * 255.0;

        *pixel = image::Rgb([r as u8, g as u8, b as u8]);
    }

    img.save("rtow.png").unwrap();
}
