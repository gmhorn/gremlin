use std::{marker::PhantomData, ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign}};

use crate::{Float, geo::Vector};

/// The CIE 1931 color space.
pub struct CIE1931;

/// Linear RGB color space.
pub struct LinearRGB;

/// A tristimulus color value, parameterized by its color space.
pub struct Color<CS> {
    // Kind of a gross implementation detail, but it already implements all the
    // operations we need...
    vals: Vector,
    _colorspace: PhantomData<CS>,
}

impl<CS> Add for Color<CS> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            vals: self.vals + rhs.vals,
            _colorspace: PhantomData,
        }
    }
}

impl<CS> AddAssign for Color<CS> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.vals += rhs.vals;
    }
}

impl<CS> Mul<Float> for Color<CS> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Self {
            vals: self.vals * rhs,
            _colorspace: PhantomData }
    }
}

impl<CS> MulAssign<Float> for Color<CS> {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        self.vals *= rhs;
    }
}

impl<CS> Div<Float> for Color<CS> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        Self{
            vals: self.vals / rhs,
            _colorspace: PhantomData,
        }
    }
}

impl<CS> DivAssign<Float> for Color<CS> {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        self.vals /= rhs;
    }
}

impl<CS> From<[Float; 3]> for Color<CS> {
    #[inline]
    fn from(vals: [Float; 3]) -> Self {
        Self { vals: vals.into(), _colorspace: PhantomData }
    }
}

/// A CIE 1931 tristimulus color value.
pub type XYZ = Color<CIE1931>;

impl XYZ {
    /// Create a new XYZ color from component values.
    #[inline]
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self { vals: Vector::new(x, y, z), _colorspace: PhantomData }
    }
}

/// A linear RGB color value.
pub type RGB = Color<LinearRGB>;

impl RGB {
    /// Create a new RGB color from component values.
    #[inline]
    pub const fn new(r: Float, g: Float, b: Float) -> Self {
        Self { vals: Vector::new(r, g, b), _colorspace: PhantomData }
    }

    /// Convert linear RGB to sRGB.
    pub fn to_srgb(&self) -> [u8; 3] {
        // Convert linear RGB to sRGB by applying gamma
        let mut vals = self.vals.apply(Self::gamma);

        // If we're out of gamut, desaturate
        let min = vals.min_component();
        if min < 0.0 {
            vals -= Vector::splat(min);
        }

        // Clamp max value
        let max = vals.max_component();
        if max > 1.0 {
            vals /= max;
        }

        // Scale by 255 and convert to u8
        vals *= 255.0;
        [vals.x as u8, vals.y as u8, vals.z as u8]
    }

    // Function for taking linear RGB to sRGB.
    //
    // Values from Bruce Lindbloom's page
    // http://www.brucelindbloom.com/
    fn gamma(v: Float) -> Float {
        if v <= 0.0031308 {
            12.92 * v
        } else {
            1.055 * v.powf(0.41667) - 0.055
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn type_system() {
        let xyz1 = XYZ::new(0.25, 0.5, 0.75);
        let xyz2 = XYZ::new(0.25, 0.5, 0.75);
        let rgb1 = RGB::new(0.25, 0.5, 0.75);
        let rgb2 = RGB::new(0.25, 0.5, 0.75);

        let _ = xyz1 + xyz2;
        let _ = rgb1 + rgb2;
        // let _ = rgb1 + xyz1;
    }
}