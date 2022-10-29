use std::ops::{Add, Div, Mul, Sub, Neg};

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_traits::Float;

use super::{Point, Unit};

/// A 3-dimensional vector.
/// 
/// Vectors are interpreted as column vectors. They implement the basic algebra
/// of euclidean **R3** vector space. The basic operands for addition, 
/// subtraction, negation, and scalar multiplication/division are implemented.
/// The assignment operands (`+=`, `-=`, etc.) are not implemented. Generally
/// speaking, these are intended to be stack-allocated, highly inline-able, and
/// extremely cheap to copy. But if it turns out that implementing the mutator
/// ops improve ergonomics or performance, that should be easy enough.
/// 
/// Vectors, like most primitives in the [`geo`][crate::geo] package, are 
/// parameterized over the underlying field. In practice, only `f64` and `f32`
/// will be useful, since almost all functions use [`num_traits::Float`] as
/// their generic bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vector<F> {
    /// Construct a new vector with the given components.
    #[inline]
    pub const fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

    /// Construct a new vector with all components equal.
    #[inline]
    pub const fn splat(n: F) -> Self {
        Self::new(n, n, n)
    }

    /// Construct a new vector of length 1 along the x-axis.
    #[inline]
    pub fn x_axis() -> Self {
        Unit::x_axis().into()
    }

    /// Construct a new vector of length 1 along the y-axis.
    #[inline]
    pub fn y_axis() -> Self {
        Unit::y_axis().into()
    }

    /// Construct a new vector of length 1 along the z-axis.
    #[inline]
    pub fn z_axis() -> Self {
        Unit::z_axis().into()
    }

    /// Construct a new vector that is the component-wise minimum of the two.
    #[inline]
    pub fn min(a: Self, b: Self) -> Self {
        Self::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }

    /// Construct a new vector that is the component-wise maximum of the two.
    #[inline]
    pub fn max(a: Self, b: Self) -> Self {
        Self::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
    }

    /// Compute the dot product of this vector with another.
    #[inline]
    pub fn dot(self, rhs: Self) -> F {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Construct a new vector that is the cross product of this vector with
    /// another.
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    /// Compute the squared length of the vector. It is faster to compute than
    /// [`Self::len()`], so use it when you can.
    #[inline]
    pub fn len_squared(self) -> F {
        self.dot(self)
    }

    /// Compute the length of the vector.
    #[inline]
    pub fn len(self) -> F {
        self.dot(self).sqrt()
    }

    /// Normalize the vector.
    /// 
    /// Panics if vector is 0-length or otherwise ill-conditioned.
    #[inline]
    pub fn normalize(self) -> Unit<F> {
        Unit::try_from(self).unwrap()
    }

    // Returns `true` if all components are finite. Finite means neither `NaN`,
    /// positive infinity, or negative infinity.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Returns `true` if any components are `NaN`
    #[inline]
    pub fn is_nan(self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
}

// OPERATORS

impl<F: Float> Neg for Vector<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}

impl<F: Float> Add for Vector<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<F: Float> Sub for Vector<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<F: Float> Mul<F> for Vector<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::Output::new(rhs * self.x, rhs * self.y, rhs * self.z)
    }
}

impl<F: Float> Div<F> for Vector<F> {
    type Output = Self;

    // Clippy doesn't like that we're multiplying in a `div` impl, but "compute
    // the reciprical once and then do multiplication" is the lowest of low-
    // hanging fruit when it comes to this stuff, right?
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        self * rhs.recip()
    }
}

// APPROXIMATIONS

impl<F: AbsDiffEq> AbsDiffEq for Vector<F> where
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

impl<F: RelativeEq> RelativeEq for Vector<F> where
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

impl<F: UlpsEq> UlpsEq for Vector<F> where
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

// CONVERSIONS: VECTOR -> OTHER

impl<F: Float> From<Vector<F>> for [F; 3] {
    #[inline]
    fn from(v: Vector<F>) -> Self {
        [v.x, v.y, v.z]
    }
}

// CONVERSIONS: OTHER -> VECTOR

impl<F: Float> From<[F; 3]> for Vector<F> {
    #[inline]
    fn from(arr: [F; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl<F: Float> From<Point<F>> for Vector<F> {
    #[inline]
    fn from(pt: Point<F>) -> Self {
        Self::new(pt.x, pt.y, pt.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min() {
        let u = Vector::splat(1.0);
        let v = Vector::new(-1.0, 2.0, 1.0);

        assert_eq!(Vector::new(-1.0, 1.0, 1.0), Vector::min(u, v));
    }

    #[test]
    fn max() {
        let u = Vector::splat(1.0);
        let v = Vector::new(-1.0, 2.0, 1.0);

        assert_eq!(Vector::new(1.0, 2.0, 1.0), Vector::max(u, v));
    }

    #[test]
    fn dot() {
        let u = Vector::x_axis();
        let v = Vector::new(2.0, 1.0, 0.0);

        assert_eq!(2.0, u.dot(v));
    }

    #[test]
    fn cross() {
        assert_eq!(Vector::<f64>::z_axis(), Vector::x_axis().cross(Vector::y_axis()));
        assert_eq!(Vector::<f64>::x_axis(), Vector::y_axis().cross(Vector::z_axis()));
        assert_eq!(Vector::<f64>::y_axis(), Vector::z_axis().cross(Vector::x_axis()));
    }

    #[test]
    fn neg() {
        assert_eq!(Vector::splat(-1.0), -Vector::splat(1.0));
    }

    #[test]
    fn add() {
        assert_eq!(Vector::splat(3.0), Vector::splat(2.0) + Vector::splat(1.0));
    }

    #[test]
    fn mult() {
        assert_eq!(Vector::splat(1.5), Vector::splat(1.0) * 1.5);
    }

    #[test]
    fn div() {
        assert_eq!(Vector::splat(2.0), Vector::splat(8.0) / 4.0);
    }
}