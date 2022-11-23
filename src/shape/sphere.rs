use std::{cmp::Ordering, mem::swap};

use super::{Intersection, Shape};
use crate::{
    geo::{Point, Ray, Unit},
    Float,
};

/// A geometric sphere.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: Float,
}

impl Sphere {
    /// Creates a new sphere with the given center and radius.
    ///
    /// # Panics
    ///
    /// Panics if radius is not a finite, positive number.
    pub fn new(center: impl Into<Point>, radius: Float) -> Self {
        if radius.is_sign_negative() || !radius.is_normal() {
            panic!("Invalid radius {}; must be finite, positive number", radius);
        }
        Self { center: center.into(), radius: radius }
    }

    fn solve_quadratic(a: Float, b: Float, c: Float) -> Option<(Float, Float)> {
        let discr = b.powi(2) - 4.0 * a * c;
        match discr.total_cmp(&0.0) {
            Ordering::Less => None,
            Ordering::Equal => {
                let root = -0.5 * b / a;
                Some((root, root))
            }
            Ordering::Greater => {
                let q = if b > 0.0 {
                    -0.5 * (b + discr.sqrt())
                } else {
                    -0.5 * (b - discr.sqrt())
                };
                let mut x0 = q / a;
                let mut x1 = c / q;
                if x0 > x1 {
                    swap(&mut x0, &mut x1);
                }
                Some((x0, x1))
            }
        }
    }

    fn nearest_intersection(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Float> {
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
        let l = ray.origin() - self.center;

        let a = ray.direction().len_squared();
        let b = 2.0 * l.dot(ray.direction());
        let c = l.len_squared() - self.radius.powi(2);

        Self::solve_quadratic(a, b, c)
            .into_iter()
            .flat_map(|(r1, r2)| {
                let mut arr = [r1, r2];
                arr.sort_by(Float::total_cmp);
                arr
            })
            .find(|&r| t_min <= r && r <= t_max)
    }
}

impl Shape for Sphere {
    #[inline]
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        let t = self.nearest_intersection(ray, t_min, t_max)?;
        let point = ray.at(t);
        let norm = Unit::try_from(point - self.center).ok()?;
        Some(Intersection { point, norm, t })
    }

    #[inline]
    fn intersects(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        self.nearest_intersection(ray, t_min, t_max).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo::Vector;

    #[test]
    fn intersect_two_points() {
        let s = Sphere::new(Point::new(10.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point::ORIGIN, Vector::X_AXIS);

        assert_eq!(true, s.intersects(&ray, 0.0, Float::INFINITY));

        let isect = s.intersect(&ray, 0.0, Float::INFINITY).unwrap();
        assert_eq!(Point::new(9.0, 0.0, 0.0), isect.point);
        assert_eq!(-Unit::X_AXIS, isect.norm);
        assert_eq!(9.0, isect.t);
    }

    #[test]
    fn intersect_no_points() {
        let s = Sphere::new(Point::new(10.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point::ORIGIN, Vector::Y_AXIS);

        assert_eq!(false, s.intersects(&ray, 0.0, Float::INFINITY));
        assert_eq!(None, s.intersect(&ray, 0.0, Float::INFINITY));
    }

    #[test]
    fn intersect_out_of_bounds() {
        let s = Sphere::new(Point::new(10.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(Point::ORIGIN, Vector::X_AXIS);

        assert_eq!(false, s.intersects(&ray, 0.0, 7.0));
        assert_eq!(None, s.intersect(&ray, 0.0, 7.0));

        assert_eq!(false, s.intersects(&ray, 20.0, Float::INFINITY));
        assert_eq!(None, s.intersect(&ray, 20.0, Float::INFINITY));
    }
}
