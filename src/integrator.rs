use crate::{
    camera::Camera,
    color::{Color, RGB},
    film::Film,
    geo::{Ray, Vector},
    shape::{Shape, Surface},
    Float,
};
use rand::prelude::*;
use rand_distr::UnitSphere;
use rayon::prelude::*;

pub trait Integrator<Li>: Send + Sync {
    fn radiance(&self, ray: &Ray, rng: &mut impl Rng) -> Li;
}

#[derive(Debug, Default)]
pub struct Hacky {
    pub background: RGB,
    pub surfaces: Vec<Surface>,
}

impl Hacky {
    fn ray_color(&self, ray: &Ray, rng: &mut impl Rng, depth: usize) -> RGB {
        if let Some(isect) = self.surfaces.intersect(ray, 0.001, Float::INFINITY) {
            if depth < 50 {
                let rand_vec = Vector::from(UnitSphere.sample(rng));
                let target = isect.point + isect.norm.into() + rand_vec;
                let ray = Ray::new(isect.point, target - isect.point);
                self.ray_color(&ray, rng, depth + 1) * 0.5
            } else {
                RGB::from([0.0, 0.0, 0.0])
            }
        } else {
            self.background
        }
    }
}

impl Integrator<RGB> for Hacky {
    fn radiance(&self, ray: &Ray, rng: &mut impl Rng) -> RGB {
        loop {
            if let Some(isect) = self.surfaces.intersect(ray, 0.001, Float::INFINITY) {}
        }
        self.ray_color(ray, rng, 0)
    }
}

pub fn render<CS, Li>(film: &mut Film<CS>, cam: &impl Camera, integrator: &impl Integrator<Li>)
where
    Color<CS>: From<Li> + Copy + Send,
    CS: Copy,
{
    film.par_pixel_iter_mut()
        .for_each_init(rand::thread_rng, |rng, (px, py, pixel)| {
            let ray = cam.ray(px, py, rng);
            let rad = integrator.radiance(&ray, rng);
            pixel.add_sample(rad);
        });
}

fn scope(s: String) {
    let mut s = s;
    while s.len() > 1 {
        print!("{}\n", s);
        let mut chars = s.chars();
        chars.next();
        s = String::from(chars.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope() {
        scope(String::from("abcd"));
    }
}
