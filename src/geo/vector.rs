use std::ops::{Add, Div, Mul};

use super::Unit;

/// Represents a "real-valued" (`f64`-valued) vector in R3. Vectors are
/// interpreted as column vectors in homogeneous coordinates, with `w = 0`
/// identically.
///
/// Standard operations of addition, subtraction, negation, and multiplication
/// and division by a scalar are implemented. There are also functions to
/// compute the dot- and cross- product of vectors; these use methods rather
/// than overloading the `*` and `^` operators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub const X_AXIS: Vector = Vector::new(1.0, 0.0, 0.0);
    pub const Y_AXIS: Vector = Vector::new(0.0, 1.0, 0.0);
    pub const Z_AXIS: Vector = Vector::new(0.0, 0.0, 1.0);

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

    #[inline]
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn len_squared(self) -> f64 {
        self.dot(self)
    }

    #[inline]
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Option<Unit> {
        // TODO implement
        None
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
