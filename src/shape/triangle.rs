use super::{Intersection, Shape};
use crate::{geo::Ray, Float};

pub struct Triangle;

impl Shape for Triangle {
    fn intersect(&self, _ray: &Ray, _t_min: Float, _t_max: Float) -> Option<Intersection> {
        todo!()
    }
}
