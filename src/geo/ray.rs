use super::{Point, Vector};
use crate::Real;

/// A geometric ray.
///
/// Rays have an origin and a direction (not necessarily normalized). They
/// [`Point`]s along the ray may be obtained by calling [`at()`][Self::at].
#[derive(Debug)]
pub struct Ray<R> {
    origin: Point<R>,
    dir: Vector<R>,
}

impl<R: Real> Ray<R> {
    /// Construct a new ray with the given origin and direction.
    #[inline]
    pub const fn new(origin: Point<R>, dir: Vector<R>) -> Self {
        Self { origin, dir }
    }

    /// Evaluate the ray.
    #[inline]
    pub fn at(&self, t: R) -> Point<R> {
        self.origin + (self.dir * t)
    }
}
