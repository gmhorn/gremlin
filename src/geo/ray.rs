use super::{Point, Vector};
use crate::Float;

/// A geometric ray.
#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    /// Construct a new ray with the given origin and direction.
    #[inline]
    pub const fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    /// Evaluate the ray.
    #[inline]
    pub fn at(&self, t: Float) -> Point {
        self.origin + (self.direction * t)
    }

    /// The ray's direction.
    #[inline]
    pub const fn direction(&self) -> Vector {
        self.direction
    }

    /// The ray's origin.
    #[inline]
    pub const fn origin(&self) -> Point {
        self.origin
    }
}
