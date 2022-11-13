use crate::{geo::Ray, Float};
use super::{Sphere, Triangle, Shape, Intersection};

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
    fn value(&self) -> Float {
        match self {
            Self::Sphere(sphere) => sphere.value(),
            Self::Triangle(triangle) => triangle.value(),
        }
    }

    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        match self {
            Self::Sphere(sphere) => sphere.intersect(ray, t_min, t_max),
            Self::Triangle(triange) => triange.intersect(ray, t_min, t_max),
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