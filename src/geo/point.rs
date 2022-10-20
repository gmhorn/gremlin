use super::Vector;
use std::ops::{Add, Sub};

/// Represents a 3-dimensional point in space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub const ORIGIN: Self = Self::splat(0.0);

    /// Creates a new point.
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a point with all elements set to `n`.
    #[inline]
    pub const fn splat(n: f64) -> Self {
        Self::new(n, n, n)
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
