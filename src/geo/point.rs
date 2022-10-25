use std::ops::{Neg, Add, Sub};

use approx::{UlpsEq, AbsDiffEq, RelativeEq};
use num_traits::Float;

use super::Vector;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Point<F> {
    /// Construct a new point with all components `0`.
    #[inline]
    pub fn origin() -> Self {
        Self::splat(F::zero())
    }

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

    /// Compute the distance between two points.
    #[inline]
    pub fn distance(self, other: Self) -> F {
        (other - self).len()
    }

    /// Linearly interpolate between two points.
    #[inline]
    pub fn lerp(self, other: Self, t: F) -> Self {
        self + (other - self)*t
    }

    /// Compute the point midway between two points.
    #[inline]
    pub fn center(self, other: Self) -> Self {
        // TODO: is this the best way to get 1/2??
        self.lerp(other, (F::one() + F::one()).recip())
    }
}

// OPERATORS

impl<F: Float> Neg for Point<F> {
    type Output = Self;
    
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}

impl<F: Float> Add<Vector<F>> for Point<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Vector<F>) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<F: Float> Sub for Point<F> {
    type Output = Vector<F>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// APPROXIMATIONS

impl<F: AbsDiffEq> AbsDiffEq for Point<F> where
    F::Epsilon: Copy,
{
    type Epsilon = F::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        F::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        F::abs_diff_eq(&self.x, &other.x, epsilon) &&
        F::abs_diff_eq(&self.y, &other.y, epsilon) &&
        F::abs_diff_eq(&self.z, &other.z, epsilon) 
    }
}

impl<F: RelativeEq> RelativeEq for Point<F> where
    F::Epsilon: Copy,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        F::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        F::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        F::relative_eq(&self.y, &other.y, epsilon, max_relative) &&
        F::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl<F: UlpsEq> UlpsEq for Point<F> where
    F::Epsilon: Copy,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        F::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        F::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
        F::ulps_eq(&self.y, &other.y, epsilon, max_ulps) &&
        F::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}

// CONVERSIONS

impl<F: Float> From<[F; 3]> for Point<F> {
    #[inline]
    fn from(arr: [F; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl<F: Float> From<Vector<F>> for Point<F> {
    #[inline]
    fn from(v: Vector<F>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod tests {
    use approx::*;
    use super::*;

    #[test]
    fn distance() {
        let p1 = Point::origin();
        let p2 = Point::new(3.0, 4.0, 5.0);
        assert_relative_eq!(7.0710678, p1.distance(p2), max_relative = 1e-6);
    }

    #[test]
    fn lerp_and_center() {
        let p1 = Point::origin();
        let p2 = Point::splat(2.0);

        assert_eq!(p1, p1.lerp(p2, 0.0));
        assert_eq!(p2, p1.lerp(p2, 1.0));
        assert_eq!(Point::splat(1.0), p1.lerp(p2, 0.5));
        assert_eq!(Point::splat(1.0), p1.center(p2));
    }

    #[test]
    fn negation() {
        assert_eq!(Point::splat(-1.0), -Point::splat(1.0));
    }
}