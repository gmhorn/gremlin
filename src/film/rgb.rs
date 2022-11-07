use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use image::Rgb;

use crate::Float;

/// An RGB value.
///
/// Doesn't assume a particular color space. Its up to the user to apply any
/// gamma correction.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RGB(Float, Float, Float);

impl RGB {
    /// Create a new XYZ value.
    #[inline]
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self(x, y, z)
    }
}

impl Add for RGB {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for RGB {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Mul<Float> for RGB {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<Float> for RGB {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<Float> for RGB {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        self * rhs.recip()
    }
}

impl DivAssign<Float> for RGB {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        *self *= rhs.recip();
    }
}

// CONVERSIONS: RGB -> OTHER

impl From<RGB> for [Float; 3] {
    #[inline]
    fn from(rgb: RGB) -> Self {
        [rgb.0, rgb.1, rgb.2]
    }
}

impl From<RGB> for Rgb<u8> {
    /// Creates a [`u8`]-valued RGB value from this.
    #[inline]
    fn from(rgb: RGB) -> Self {
        let rgb = rgb * 255.5;
        Rgb::from([rgb.0 as u8, rgb.1 as u8, rgb.2 as u8])
    }
}
