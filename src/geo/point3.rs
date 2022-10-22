use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Point3<F> {
    /// Construct a new point with the given components.
    #[inline]
    pub const fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

    /// Construct a new point with all components equal.
    #[inline]
    pub const fn splat(n: F) -> Self {
        Self::new(n, n, n)
    }
}