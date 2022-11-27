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
    ///
    /// If intersection is found, returns the `(t_near, t_far)` ray parameter
    /// values.
    pub fn intsersects(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<(f64, f64)> {
        // https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/rayintersectionwithanaabb
        let (t0, t1) = Axis::ALL.iter().fold((t_min, t_max), |(t0, t1), &axis| {
            let inv_ray_dir = ray.direction[axis].recip();
            let mut t_near = (self.min[axis] - ray.origin[axis]) * inv_ray_dir;
            let mut t_far = (self.max[axis] - ray.origin[axis]) * inv_ray_dir;

            if t_near > t_far {
                mem::swap(&mut t_near, &mut t_far);
            }

            (t0.max(t_near), t1.min(t_far))
        });

        if t0 > t1 {
            None
        } else {
            Some((t0, t1))
        }
    }
}
