use super::{Intersection, Shape};
use crate::{geo::Ray, Float};

pub struct Triangle;

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        todo!()
    }
}
