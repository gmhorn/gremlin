use std::ops::{Add, Div, Mul, Sub};

use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<F> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vec3<F> {
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
            x: (self.y * rhs.z) - (self.z - rhs.y),
            y: (self.z * rhs.x) - (self.x - rhs.z),
            z: (self.x * rhs.y) - (self.y - rhs.x),
        }
    }

    /// Compute the squared length of the vector. It is faster to compute than
    /// [`len()`], so use it when you can.
    #[inline]
    pub fn len_squared(self) -> F {
        self.dot(self)
    }

    /// Returns the length of the vector.
    #[inline]
    pub fn len(self) -> F {
        self.dot(self).sqrt()
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

impl<F: Float> Add for Vec3<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<F: Float> Sub for Vec3<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<F: Float> Mul<F> for Vec3<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::Output::new(rhs * self.x, rhs * self.y, rhs * self.z)
    }
}

impl<F: Float> Div<F> for Vec3<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self::Output {
        self * rhs.recip()
    }
}

// CONVERSIONS

impl<F: Float> From<[F; 3]> for Vec3<F> {
    fn from(_: [F; 3]) -> Self {
        todo!()
    }
}