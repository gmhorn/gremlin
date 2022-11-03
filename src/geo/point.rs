use super::Vector;
use crate::Float;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::{Add, Neg, Sub};

/// A 3-dimensional point in euclidean space.
///
/// Although points and vectors are both elements of **R3** euclidean space,
/// points support far fewer operations. The reason there are separate types at
/// all is mostly for API clarity:
///
/// * Allowing the other packages to use both points and vectors often makes it
///   clearer what something's intended purpose and behavior is.
/// * Points and vectors transform differently in homogeneous coordinates, which
///   is easier to enforce with them being separate types.
///
/// Conceptually, points are "just" elements of **R3** while vectors are
/// elements of the underlying vector space. So the only operations points
/// support are:
///
/// * **Negation**: Flips the point about the origin.
/// * **Subtraction**: `p1 - p2` returns the vector from `p1` to `p2`.
/// * **Translation**: `p + v` returns another point that is the translation of
///   `p` by the vector `v`
/// * Some convenience functions like [`Self::distance()`], [`Self::lerp()`],
///   and [`Self::center()`].
///
/// Points, like most primitives in the [`geo`][crate::geo] package, are
/// parameterized over the underlying field. In practice, only `f64` and `f32`
/// will be useful, since almost all functions use [`num_traits::Float`] as
/// their generic bound.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Point {
    /// The origin.
    pub const ORIGIN: Self = Self::splat(0.0);

    /// Construct a new point with the given components.
    #[inline]
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    /// Construct a new point with all components equal.
    #[inline]
    pub const fn splat(n: Float) -> Self {
        Self::new(n, n, n)
    }

    /// Construct a new point that is the component-wise minimum of the two.
    #[inline]
    pub fn min(a: Self, b: Self) -> Self {
        Self::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }

    /// Construct a new point that is the component-wise maximum of the two.
    #[inline]
    pub fn max(a: Self, b: Self) -> Self {
        Self::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
    }

    /// Compute the distance between two points.
    #[inline]
    pub fn distance(self, other: Self) -> Float {
        (other - self).len()
    }

    /// Linearly interpolate between two points.
    #[inline]
    pub fn lerp(self, other: Self, t: Float) -> Self {
        self + (other - self) * t
    }

    /// Compute the point midway between two points.
    #[inline]
    pub fn center(self, other: Self) -> Self {
        self.lerp(other, 0.5)
    }
}

// OPERATORS

impl Neg for Point {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg())
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

// APPROXIMATIONS

impl AbsDiffEq for Point {
    type Epsilon = Float;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        Float::default_epsilon()
    }

    #[rustfmt::skip]
    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        Float::abs_diff_eq(&self.x, &other.x, epsilon) &&
        Float::abs_diff_eq(&self.y, &other.y, epsilon) &&
        Float::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl RelativeEq for Point {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        Float::default_max_relative()
    }

    #[rustfmt::skip]
    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        Float::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        Float::relative_eq(&self.y, &other.y, epsilon, max_relative) &&
        Float::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl UlpsEq for Point {
    #[inline]
    fn default_max_ulps() -> u32 {
        Float::default_max_ulps()
    }

    #[rustfmt::skip]
    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        Float::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
        Float::ulps_eq(&self.y, &other.y, epsilon, max_ulps) &&
        Float::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}

// CONVERSIONS

impl From<[Float; 3]> for Point {
    #[inline]
    fn from(arr: [Float; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl From<Vector> for Point {
    #[inline]
    fn from(v: Vector) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn min() {
        let p = Point::splat(1.0);
        let q = Point::new(-1.0, 2.0, 1.0);

        assert_eq!(Point::new(-1.0, 1.0, 1.0), Point::min(p, q));
    }

    #[test]
    fn max() {
        let p = Point::splat(1.0);
        let q = Point::new(-1.0, 2.0, 1.0);

        assert_eq!(Point::new(1.0, 2.0, 1.0), Point::max(p, q));
    }

    #[test]
    fn distance() {
        let p = Point::ORIGIN;
        let q = Point::new(3.0, 4.0, 5.0);
        assert_relative_eq!(7.0710678, p.distance(q), max_relative = 1e-6);
    }

    #[test]
    fn lerp_and_center() {
        let p = Point::ORIGIN;
        let q = Point::splat(2.0);

        assert_eq!(p, p.lerp(q, 0.0));
        assert_eq!(q, p.lerp(q, 1.0));
        assert_eq!(Point::splat(1.0), p.lerp(q, 0.5));
        assert_eq!(Point::splat(1.0), p.center(q));
    }

    #[test]
    fn negation() {
        assert_eq!(Point::splat(-1.0), -Point::splat(1.0));
    }
}
