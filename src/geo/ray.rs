use crate::Float;

use super::{Point, Vector};

/// A geometric ray.
///
/// Rays have an origin and a direction (not necessarily normalized). The
/// [`Point`]s along the ray may be obtained by calling [`at()`][Self::at].
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
}
