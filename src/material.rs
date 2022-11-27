use rand::Rng;

use crate::{color::RGB, geo::Ray, shape::Intersection};

mod lambertian;
pub use lambertian::*;

pub trait BSDF {
    fn scatter(&self, ray: &Ray, isec: &Intersection, rng: &mut impl Rng) -> Option<(RGB, Ray)>;
}

pub enum Material {}
