use super::{Intersection, Shape};
use crate::{geo::Ray, Float};

pub struct Sphere;

impl Shape for Sphere {
    fn value(&self) -> Float {
        2.0
    }

    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        todo!()
    }
}
