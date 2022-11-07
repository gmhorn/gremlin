use gremlin::{
    film::{RGBBuffer, RGBFilm, Save, SpectralFilm, RGB},
    spectrum::{self, Sampled},
};

fn main() {
    let buf = RGBBuffer::new(10, 5);
    buf.save_image("patfooh").unwrap();

    let spec = Sampled::from(|w| spectrum::blackbody(6500.0, w));
    let mut film = SpectralFilm::new(10, 5);
    film.get_pixel_mut(1, 0).add_sample(spec);

    let mut film = RGBFilm::new(10, 5);
    film.get_pixel_mut(1, 0).add_sample(RGB::new(1.0, 0.0, 0.0));
}
