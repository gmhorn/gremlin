use crate::{camera::Camera, color::Color, film::Film, geo::Ray};
use rayon::prelude::*;

pub trait Integrator<Li>: Send + Sync {
    fn radiance(&self, ray: &Ray) -> Li;
}

pub struct PathTracer {
    // immutable things, so tracer can be send + sync
    // scene
    // lights
    // camera
}

pub fn render<CS, Li>(film: &mut Film<CS>, cam: &impl Camera, integrator: &impl Integrator<Li>)
where
    Color<CS>: From<Li> + Copy + Send,
    CS: Copy,
{
    film.par_pixel_iter_mut()
        .for_each_init(rand::thread_rng, |rng, (px, py, pixel)| {
            let ray = cam.ray(px, py, rng);
            let rad = integrator.radiance(&ray);
            pixel.add_sample(rad);
        });
}
