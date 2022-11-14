use super::{Intersection, Shape, Surface};
use crate::{geo::Ray, Float};

pub struct DynamicAggregate(Vec<Box<dyn Shape>>);

impl DynamicAggregate {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add<S: Shape + 'static>(&mut self, s: S) {
        self.0.push(Box::new(s))
    }
}

impl Default for DynamicAggregate {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape for DynamicAggregate {
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        todo!()
    }
}

pub struct SurfaceAggregate(Vec<Surface>);

impl SurfaceAggregate {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add<S: Into<Surface>>(&mut self, s: S) {
        self.0.push(s.into())
    }
}

impl Default for SurfaceAggregate {
    fn default() -> Self {
        Self::new()
    }
}
