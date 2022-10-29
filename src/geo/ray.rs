use num_traits::Float;

use super::{Point, Vector};

/// A geometric ray.
/// 
/// Rays have an origin and a direction (not necessarily normalized). They
/// [`Point`]s along the ray may be obtained by calling [`at()`][Self::at].
#[derive(Debug)]
pub struct Ray<F> {
    origin: Point<F>,
    dir: Vector<F>,
}

impl<F: Float> Ray<F> {
    /// Construct a new ray with the given origin and direction.
    #[inline]
    pub const fn new(origin: Point<F>, dir: Vector<F>) -> Self {
        Self { origin, dir }
    }

    /// Evaluate the ray.
    #[inline]
    pub fn at(&self, t: F) -> Point<F> {
        self.origin + (self.dir * t)
    }
}