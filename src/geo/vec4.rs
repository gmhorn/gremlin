use std::ops::Mul;

use num_traits::Float;

use super::{Vec3, Point3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Vec4<F> {
    pub(super) x: F,
    pub(super) y: F,
    pub(super) z: F,
    pub(super) w: F,
}

impl<F: Float> Vec4<F> {
    #[inline]
    pub const fn new(x: F, y: F, z: F, w: F) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub fn x_axis() -> Self {
        Self::new(F::one(), F::zero(), F::zero(), F::zero())
    }

    #[inline]
    pub fn y_axis() -> Self {
        Self::new(F::zero(), F::one(), F::zero(), F::zero())
    }

    #[inline]
    pub fn z_axis() -> Self {
        Self::new(F::zero(), F::zero(), F::one(), F::zero())
    }

    #[inline]
    pub fn w_axis() -> Self {
        Self::new(F::zero(), F::zero(), F::zero(), F::one())
    }
}

// OPERATORS

impl<F: Float> Mul<F> for Vec4<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        Self::Output::new(rhs * self.x, rhs * self.y, rhs * self.z, rhs * self.w)
    }
}

// CONVERSIONS

impl<F: Float> From<Vec3<F>> for Vec4<F> {
    #[inline]
    fn from(v: Vec3<F>) -> Self {
        Self::new(v.x, v.y, v.z, F::zero())
    }
}

impl<F: Float> From<Point3<F>> for Vec4<F> {
    #[inline]
    fn from(p: Point3<F>) -> Self {
        Self::new(p.x, p.y, p.z, F::one())
    }
}