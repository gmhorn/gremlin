use std::ops::{Add, Div, Mul, Sub};

use super::UnitOld;

/// Represents a 3-dimensional vector. Vectors are interpreted as column vectors
/// in homogeneous coordinates, with `w = 0` identically.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorOld {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VectorOld {
    pub const X_AXIS: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y_AXIS: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z_AXIS: Self = Self::new(0.0, 0.0, 1.0);

    /// Construct a new vector directly from its component values.
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Construct a new vector with all components equal.
    #[inline]
    pub const fn splat(n: f64) -> Self {
        Self::new(n, n, n)
    }

    /// Construct a new vector that is the component-wise minimum of the two
    /// vectors.
    #[inline]
    pub fn min(a: Self, b: Self) -> Self {
        Self::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }

    /// Construct a new vector that is the component-wise maximum of the two
    /// vectors.
    #[inline]
    pub fn max(a: &Self, b: &Self) -> Self {
        Self::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
    }

    /// Computes the dot product of this vector with another.
    #[inline]
    pub fn dot(self, rhs: Self) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    /// Returns a vector representing the cross product of this vector with
    /// another.
    #[inline]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: (self.y * rhs.z) - (self.z - rhs.y),
            y: (self.z * rhs.x) - (self.x - rhs.z),
            z: (self.x * rhs.y) - (self.y - rhs.x),
        }
    }

    /// Returns the squared length of the vector. It is faster to compute than
    /// `len()`, so use it when you can.
    #[inline]
    pub fn len_squared(self) -> f64 {
        self.dot(self)
    }

    /// Returns the length of the vector.
    #[inline]
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Normalizes the vector. For valid results, must not be called when `self`
    /// is 0 or very close to 0.
    ///
    /// If debug assertions are turned on, panics if the returned value is not
    /// finite. But in release builds, will silently return a garbage value.
    ///
    /// See [`Self::try_normalize`] for a safer alternative.
    #[inline]
    pub fn normalize(self) -> UnitOld {
        let u = self / self.len();
        debug_assert!(u.is_finite());
        UnitOld::new(u.x, u.y, u.z)
    }

    /// Returns the normalized unit vector if possible, else `None`.
    #[inline]
    pub fn try_normalize(self) -> Option<UnitOld> {
        let recip = self.len().recip();
        if recip.is_finite() && recip > 0.0 {
            let u = self * recip;
            Some(UnitOld::new(u.x, u.y, u.z))
        } else {
            None
        }
    }

    /// Returns `true` if all components are finite. Finite means neither `NaN`,
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

impl Add for VectorOld {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for VectorOld {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for VectorOld {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(rhs * self.x, rhs * self.y, rhs * self.z)
    }
}

impl Mul<VectorOld> for f64 {
    type Output = VectorOld;

    #[inline]
    fn mul(self, rhs: VectorOld) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for VectorOld {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl From<UnitOld> for VectorOld {
    #[inline]
    fn from(u: UnitOld) -> Self {
        Self::new(u.x, u.y, u.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_min() {
        let a = VectorOld::splat(1.0);
        let b = VectorOld::new(-1.0, 2.0, 1.0);
        let expect = VectorOld::new(-1.0, 1.0, 1.0);
        assert_eq!(expect, VectorOld::min(a, b))
    }

    #[test]
    fn vector_add() {
        let a = VectorOld::splat(1.0);
        let b = VectorOld::new(1.0, 2.0, 3.0);
        let expect = VectorOld::new(2.0, 3.0, 4.0);

        assert_eq!(expect, a + b);
    }

    #[test]
    fn vector_mul() {
        let v = VectorOld::new(1.0, 2.0, 3.0);
        let n = 2.0;
        let expect = VectorOld::new(2.0, 4.0, 6.0);

        assert_eq!(expect, n * v);
        assert_eq!(expect, v * n);
    }

    #[test]
    fn vector_div() {
        let v = VectorOld::new(2.0, 4.0, 6.0);
        let n = 2.0;
        let expect = VectorOld::new(1.0, 2.0, 3.0);

        assert_eq!(expect, v / n);
    }
}
