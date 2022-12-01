use super::{Component, Point, Ray};
use crate::Float;
use std::mem;

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
        let (t0, t1) = Component::XYZ.iter().fold((t_min, t_max), |(t0, t1), &i| {
            let inv_ray_dir = ray.direction[i].recip();
            let mut t_near = (self.min[i] - ray.origin[i]) * inv_ray_dir;
            let mut t_far = (self.max[i] - ray.origin[i]) * inv_ray_dir;

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

#[cfg(test)]
mod tests {
    use crate::geo::Vector;
    use super::*;

    #[test]
    fn intersects() {
        let bounds = Bounds::from_corners(Point::splat(-1.0), Point::splat(1.0));

        let ray = Ray::new(Point::new(0.0, 0.0, -10.0), Vector::Z_AXIS);
        assert_eq!(Some((9.0, 11.0)), bounds.intsersects(&ray, 0.0, Float::INFINITY));

        let ray = Ray::new(Point::new(0.0, 0.0, -10.0), Vector::Y_AXIS);
        assert_eq!(None, bounds.intsersects(&ray, 0.0, Float::INFINITY));
    }
}