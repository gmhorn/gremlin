use std::mem;

use super::{Axis, Point, Ray};
use crate::Float;

/// An axis-aligned bounding box.
#[derive(Debug)]
pub struct Bounds {
    min: Point,
    max: Point,
}

impl Bounds {
    /// Create a new bounds from the given corner points.
    pub fn from_corners(p1: Point, p2: Point) -> Self {
        Self {
            min: Point::min(p1, p2),
            max: Point::max(p1, p2),
        }
    }

    /// Test a ray for intersection.
    pub fn intsersects(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        // https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/rayintersectionwithanaabb
        let mut t0 = t_min;
        let mut t1 = t_max;
        let (t0, t1) = [Axis::X, Axis::Y, Axis::Z]
            .into_iter()
            .fold((t_min, t_max), |(t0, t1), axis| (t0, t1));
        // for i in [Axis::X, Axis::Y, Axis::Z] {
        //     let inv_ray_dir = ray.direction[i].recip();
        //     let mut t_near = (self.min[i] - ray.origin[i]) * inv_ray_dir;
        //     let mut t_far = (self.max[i] - ray.origin[i]) * inv_ray_dir;

        //     if t_near > t_far {
        //         mem::swap(&mut t_near, &mut t_far);
        //     }

        //     t0 = t_near.max(t0);
        //     t1 = t_far.min(t1);
        // }

        todo!()
    }
}
