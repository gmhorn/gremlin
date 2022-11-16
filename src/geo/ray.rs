use super::{Point, Vector};
use crate::Float;

/// A geometric ray.
#[derive(Debug)]
pub struct Ray {
    origin: Point,
    dir: Vector,
}

impl Ray {
    /// Construct a new ray with the given origin and direction.
    #[inline]
    pub const fn new(origin: Point, dir: Vector) -> Self {
        Self { origin, dir }
    }

    /// Evaluate the ray.
    #[inline]
    pub fn at(&self, t: Float) -> Point {
        self.origin + (self.dir * t)
    }

    /// The ray's direction.
    #[inline]
    pub const fn direction(&self) -> Vector {
        self.dir
    }

    /// The ray's origin.
    #[inline]
    pub const fn origin(&self) -> Point {
        self.origin
    }
}
