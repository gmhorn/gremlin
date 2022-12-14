use super::{Component, Point, Unit};
use crate::Float;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

/// A 3-dimensional vector.
///
/// Vectors are interpreted as column vectors. They implement the basic algebra
/// of euclidean **R3** vector space. The basic operands for addition,
/// subtraction, negation, and scalar multiplication/division are implemented.
/// The assignment operands (`+=`, `-=`, etc.) are not implemented. Generally
/// speaking, these are intended to be stack-allocated, highly inline-able, and
/// extremely cheap to copy. But if it turns out that implementing the mutator
/// ops improve ergonomics or performance, that should be easy enough.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vector {
    /// A vector of length 1 in the x direction.
    pub const X_AXIS: Vector = Vector::new(1.0, 0.0, 0.0);

    /// A vector of length 1 in the y direction.
    pub const Y_AXIS: Vector = Vector::new(0.0, 1.0, 0.0);

    /// A vector of length 1 in the z direction.
    pub const Z_AXIS: Vector = Vector::new(0.0, 0.0, 1.0);

    /// The zero-vector.
    pub const ZERO: Vector = Vector::new(0.0, 0.0, 0.0);

    /// Construct a new vector with the given components.
    #[inline]
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    /// Construct a new vector with all components equal.
    #[inline]
    pub const fn splat(n: Float) -> Self {
        Self::new(n, n, n)
    }

    /// Construct a new vector that is the component-wise minimum of the two.
    #[inline]
    pub fn min(a: Self, b: Self) -> Self {
        Self::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }

    /// Fetch the minimum component of this vector.
    #[inline]
    pub fn min_component(&self) -> Float {
        self.x.min(self.y).min(self.z)
    }

    /// Construct a new vector that is the component-wise maximum of the two.
    #[inline]
    pub fn max(a: Self, b: Self) -> Self {
        Self::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
    }

    /// Fetch the maximum component of this vector.
    #[inline]
    pub fn max_component(&self) -> Float {
        self.x.max(self.y).max(self.z)
    }

    /// Compute the dot product of this vector with another.
    #[inline]
    pub fn dot(&self, rhs: Self) -> Float {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Construct a new vector that is the cross product of this vector with
    /// another.
    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    /// Construct a new vectory by applying a function to the components of this
    /// vector.
    #[inline]
    pub fn apply<F>(&self, f: F) -> Self
    where
        F: Fn(Float) -> Float,
    {
        Self::new(f(self.x), f(self.y), f(self.z))
    }

    /// Compute the squared length of the vector. It is faster to compute than
    /// [`Self::len()`], so use it when you can.
    #[inline]
    pub fn len_squared(self) -> Float {
        self.dot(self)
    }

    /// Compute the length of the vector.
    #[inline]
    pub fn len(self) -> Float {
        self.dot(self).sqrt()
    }

    /// Normalize the vector.
    ///
    /// Panics if vector is 0-length or otherwise ill-conditioned.
    #[inline]
    pub fn normalize(self) -> Unit {
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

impl Neg for Vector {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}

impl Add for Vector {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Float> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output::new(rhs * self.x, rhs * self.y, rhs * self.z)
    }
}

impl MulAssign<Float> for Vector {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vector> for Float {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<Float> for Vector {
    type Output = Self;

    // Clippy doesn't like that we're multiplying in a `div` impl, but "compute
    // the reciprical once and then do multiplication" is the lowest of low-
    // hanging fruit when it comes to this stuff, right?
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        self * rhs.recip()
    }
}

impl DivAssign<Float> for Vector {
    // Clippy doesn't like that we're multiplying in a `div` impl, but "compute
    // the reciprical once and then do multiplication" is the lowest of low-
    // hanging fruit when it comes to this stuff, right?
    #[allow(clippy::suspicious_op_assign_impl)]
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        *self *= rhs.recip();
    }
}

impl Index<Component> for Vector {
    type Output = Float;

    #[inline]
    fn index(&self, index: Component) -> &Self::Output {
        match index {
            Component::X => &self.x,
            Component::Y => &self.y,
            Component::Z => &self.z,
        }
    }
}

impl IndexMut<Component> for Vector {
    #[inline]
    fn index_mut(&mut self, index: Component) -> &mut Self::Output {
        match index {
            Component::X => &mut self.x,
            Component::Y => &mut self.y,
            Component::Z => &mut self.z,
        }
    }
}

// APPROXIMATIONS

impl AbsDiffEq for Vector {
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

impl RelativeEq for Vector {
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

impl UlpsEq for Vector {
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

// CONVERSIONS: VECTOR -> OTHER

impl From<Vector> for [Float; 3] {
    #[inline]
    fn from(v: Vector) -> Self {
        [v.x, v.y, v.z]
    }
}

// CONVERSIONS: OTHER -> VECTOR

impl From<[Float; 3]> for Vector {
    #[inline]
    fn from(arr: [Float; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}

impl From<Point> for Vector {
    #[inline]
    fn from(pt: Point) -> Self {
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
        let u = Vector::X_AXIS;
        let v = Vector::new(2.0, 1.0, 0.0);

        assert_eq!(2.0, u.dot(v));
    }

    #[rustfmt::skip]
    #[test]
    fn cross() {
        assert_eq!(Vector::Z_AXIS, Vector::X_AXIS.cross(Vector::Y_AXIS));
        assert_eq!(Vector::X_AXIS, Vector::Y_AXIS.cross(Vector::Z_AXIS));
        assert_eq!(Vector::Y_AXIS, Vector::Z_AXIS.cross(Vector::X_AXIS));
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
