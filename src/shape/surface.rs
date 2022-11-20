use super::{Intersection, Shape, Sphere, Triangle};
use crate::{geo::Ray, Float};

/// A surface that supports ray-object intersection.
///
/// This is essentially a polymorphic enum over the various [`Shape`] trait
/// implementations. Done to allow fast static dispatch (with matching) vs.
/// comparable slower dynamic dispatch (via [`Box<dyn Shape>`] or similar).
///
/// [`Shape`]: crate::shape::Shape
pub enum Surface {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Shape for Surface {
    #[inline]
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        match self {
            Self::Sphere(s) => s.intersect(ray, t_min, t_max),
            Self::Triangle(t) => t.intersect(ray, t_min, t_max),
        }
    }

    #[inline]
    fn intersects(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        match self {
            Self::Sphere(s) => s.intersects(ray, t_min, t_max),
            Self::Triangle(t) => t.intersects(ray, t_min, t_max),
        }
    }
}

impl From<Sphere> for Surface {
    fn from(sphere: Sphere) -> Self {
        Self::Sphere(sphere)
    }
}

impl From<Triangle> for Surface {
    fn from(triangle: Triangle) -> Self {
        Self::Triangle(triangle)
    }
}
