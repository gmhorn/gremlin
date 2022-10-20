use std::ops::{Add, Div, Mul, Sub};

use super::Unit;

/// Represents a 3-dimensional vector. Vectors are interpreted as column vectors
/// in homogeneous coordinates, with `w = 0` identically.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
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
        Self { x: n, y: n, z: n }
    }

    /// Construct a new vector that is the component-wise minimum of the two
    /// vectors.
    #[inline]
    pub fn min(a: Self, b: Self) -> Self {
        Self {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
            z: a.z.min(b.z),
        }
    }

    /// Construct a new vector that is the component-wise maximum of the two
    /// vectors.
    #[inline]
    pub fn max(a: &Self, b: &Self) -> Self {
        Self {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
            z: a.z.max(b.z),
        }
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

    /// Normalizes the vector, returning either the unit or `None`.
    #[inline]
    pub fn normalize(self) -> Option<Unit> {
        let u = self * self.len().recip();
        if u.is_finite() {
            return Some(Unit::new(u.x, u.y, u.z))
        }
        None
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

impl Add for Vector {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl From<Unit> for Vector {
    #[inline]
    fn from(u: Unit) -> Self {
        Self::new(u.x, u.y, u.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_min() {
        let a = Vector::splat(1.0);
        let b = Vector::new(-1.0, 2.0, 1.0);
        let expect = Vector::new(-1.0, 1.0, 1.0);
        assert_eq!(expect, Vector::min(a, b))
    }

    #[test]
    fn vector_add() {
        let a = Vector::splat(1.0);
        let b = Vector::new(1.0, 2.0, 3.0);
        let expect = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(expect, a + b);
    }

    #[test]
    fn vector_mul() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let n = 2.0;
        let expect = Vector::new(2.0, 4.0, 6.0);

        assert_eq!(expect, n * v);
        assert_eq!(expect, v * n);
    }

    #[test]
    fn vector_div() {
        let v = Vector::new(2.0, 4.0, 6.0);
        let n = 2.0;
        let expect = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(expect, v / n);
    }
}
