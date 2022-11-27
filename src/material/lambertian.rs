use crate::{
    color::RGB,
    geo::{Ray, Vector},
    shape::Intersection,
};
use approx::relative_eq;
use rand::prelude::*;
use rand_distr::UnitSphere;

use super::BSDF;

pub struct Lambertian(RGB);

impl Lambertian {
    pub const fn new(rgb: RGB) -> Self {
        Self(rgb)
    }
}

impl BSDF for Lambertian {
    fn scatter(&self, _ray: &Ray, isect: &Intersection, rng: &mut impl Rng) -> Option<(RGB, Ray)> {
        let mut scatter_dir = Vector::from(UnitSphere.sample(rng)) + isect.norm.into();

        // Catch degenrate scatter direction
        if relative_eq!(scatter_dir, Vector::ZERO, max_relative = 1e-8) {
            scatter_dir = isect.norm.into();
        }

        let scattered = Ray::new(isect.point, scatter_dir);
        Option::Some((self.0, scattered))
    }
}
