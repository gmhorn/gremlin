//! Color spaces and conversions.
//!
//! The base [`Color`] struct represents a tristimulus color value in a specific
//! color space. Each component has a gamut of `[0, 1]`. Basic arithmetic on
//! color values within a color space is supported, while preventing arithmetic
//! on color values in different spaces.
//!
//! Two color spaces are supported: [`CIE1931`] and [`LinearRGB`]. Convenience
//! typedefs ([`XYZ`] and [`RGB`], respectively) make it easy to construct and
//! refer to values in these spaces.
//!
//! ```
//! use gremlin::color::{RGB, XYZ};
//!
//! let mut rgb = RGB::from([0.0, 0.5, 1.0]);
//! let mut xyz = XYZ::from([0.5, 0.3, 0.0]);
//!
//! rgb /= 2.0;
//! xyz *= 2.0;
//!
//! // won't compile!
//! //let invalid = rgb + xyz;
//! ```

use crate::{geo::Vector, spectrum::Sampled, Float};
use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

/// The CIE 1931 color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CIE1931;

/// Linear RGB color space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LinearRGB;

/// A tristimulus color value, parameterized by its color space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color<CS> {
    // Kind of a gross implementation detail, but it already implements all the
    // operations we need...
    vals: Vector,
    _colorspace: PhantomData<CS>,
}

impl<CS> Default for Color<CS> {
    #[inline]
    fn default() -> Self {
        Self {
            vals: Vector::splat(0.0),
            _colorspace: PhantomData,
        }
    }
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
            _colorspace: PhantomData,
        }
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
        Self {
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
        Self {
            vals: vals.into(),
            _colorspace: PhantomData,
        }
    }
}

/// A CIE 1931 tristimulus color value.
pub type XYZ = Color<CIE1931>;

// TODO: Consider moving to Spectrum module?
impl From<Sampled> for XYZ {
    /// Converts a sampled spectrum to XYZ by integrating against the CIE color-
    /// matching curves.
    #[inline]
    fn from(sampled: Sampled) -> Self {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for (i, val) in sampled.iter().enumerate() {
            x += val * consts::CIE_X[i];
            y += val * consts::CIE_Y[i];
            z += val * consts::CIE_Z[i];
        }

        XYZ::from([x, y, z]) * consts::CIE_NORM
    }
}

/// A linear RGB color value.
pub type RGB = Color<LinearRGB>;

impl RGB {
    /// Convert linear RGB to sRGB.
    pub fn to_srgb(&self) -> [u8; 3] {
        // Implementation note:
        //
        // This is more-or-less a direct port of John Walker's code from his
        // _Colour Rendering of Spectra_ page:
        // * <https://www.fourmilab.ch/documents/specrend/>
        // * <https://www.fourmilab.ch/documents/specrend/specrend.c>

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

impl From<XYZ> for RGB {
    /// Converts an XYZ to linear RGB.
    #[inline]
    fn from(xyz: XYZ) -> Self {
        Self {
            vals: consts::XYZ_TO_RGB * xyz.vals,
            _colorspace: PhantomData,
        }
    }
}

impl From<RGB> for [u8; 3] {
    /// Converts linear RGB to 8-bit sRGB array.
    #[inline]
    fn from(rgb: RGB) -> Self {
        rgb.to_srgb()
    }
}

mod consts {
    use crate::{geo::Matrix, spectrum::Sampled, Float};

    // Matrix for taking XYZ to linear RGB
    //
    // Values from Bruce Lindbloom's page
    // http://www.brucelindbloom.com/
    #[rustfmt::skip]
    pub const XYZ_TO_RGB: Matrix = Matrix::new([
        [ 3.2404542, -1.5371385, -0.4985314, 0.0],
        [-0.9692660,  1.8760108,  0.0415560, 0.0],
        [ 0.0556434, -0.2040259,  1.0572252, 0.0],
        [ 0.0,        0.0,        0.0,       0.0]
    ]);

    #[rustfmt::skip]
    pub const CIE_X: Sampled = Sampled::new([
        1.368000e-03, 2.236000e-03, 4.243000e-03, 7.650000e-03, 1.431000e-02,
        2.319000e-02, 4.351000e-02, 7.763000e-02, 1.343800e-01, 2.147700e-01,
        2.839000e-01, 3.285000e-01, 3.482800e-01, 3.480600e-01, 3.362000e-01,
        3.187000e-01, 2.908000e-01, 2.511000e-01, 1.953600e-01, 1.421000e-01,
        9.564000e-02, 5.795001e-02, 3.201000e-02, 1.470000e-02, 4.900000e-03,
        2.400000e-03, 9.300000e-03, 2.910000e-02, 6.327000e-02, 1.096000e-01,
        1.655000e-01, 2.257499e-01, 2.904000e-01, 3.597000e-01, 4.334499e-01,
        5.120501e-01, 5.945000e-01, 6.784000e-01, 7.621000e-01, 8.425000e-01,
        9.163000e-01, 9.786000e-01, 1.026300e+00, 1.056700e+00, 1.062200e+00,
        1.045600e+00, 1.002600e+00, 9.384000e-01, 8.544499e-01, 7.514000e-01,
        6.424000e-01, 5.419000e-01, 4.479000e-01, 3.608000e-01, 2.835000e-01,
        2.187000e-01, 1.649000e-01, 1.212000e-01, 8.740000e-02, 6.360000e-02,
        4.677000e-02, 3.290000e-02, 2.270000e-02, 1.584000e-02, 1.135916e-02,
        8.110916e-03, 5.790346e-03, 4.109457e-03, 2.899327e-03, 2.049190e-03,
        1.439971e-03, 9.999493e-04, 6.900786e-04, 4.760213e-04, 3.323011e-04,
        2.348261e-04, 1.661505e-04, 1.174130e-04, 8.307527e-05, 5.870652e-05,
    ]);

    #[rustfmt::skip]
    pub const CIE_Y: Sampled = Sampled::new([
        3.900000e-05, 6.400000e-05, 1.200000e-04, 2.170000e-04, 3.960000e-04,
        6.400000e-04, 1.210000e-03, 2.180000e-03, 4.000000e-03, 7.300000e-03,
        1.160000e-02, 1.684000e-02, 2.300000e-02, 2.980000e-02, 3.800000e-02,
        4.800000e-02, 6.000000e-02, 7.390000e-02, 9.098000e-02, 1.126000e-01,
        1.390200e-01, 1.693000e-01, 2.080200e-01, 2.586000e-01, 3.230000e-01,
        4.073000e-01, 5.030000e-01, 6.082000e-01, 7.100000e-01, 7.932000e-01,
        8.620000e-01, 9.148501e-01, 9.540000e-01, 9.803000e-01, 9.949501e-01,
        1.000000e+00, 9.950000e-01, 9.786000e-01, 9.520000e-01, 9.154000e-01,
        8.700000e-01, 8.163000e-01, 7.570000e-01, 6.949000e-01, 6.310000e-01,
        5.668000e-01, 5.030000e-01, 4.412000e-01, 3.810000e-01, 3.210000e-01,
        2.650000e-01, 2.170000e-01, 1.750000e-01, 1.382000e-01, 1.070000e-01,
        8.160000e-02, 6.100000e-02, 4.458000e-02, 3.200000e-02, 2.320000e-02,
        1.700000e-02, 1.192000e-02, 8.210000e-03, 5.723000e-03, 4.102000e-03,
        2.929000e-03, 2.091000e-03, 1.484000e-03, 1.047000e-03, 7.400000e-04,
        5.200000e-04, 3.611000e-04, 2.492000e-04, 1.719000e-04, 1.200000e-04,
        8.480000e-05, 6.000000e-05, 4.240000e-05, 3.000000e-05, 2.120000e-05,
    ]);

    #[rustfmt::skip]
    pub const CIE_Z: Sampled = Sampled::new([
        6.4500010e-03, 1.0549990e-02, 2.0050010e-02, 3.6210000e-02, 6.7850010e-02,
        1.1020000e-01, 2.0740000e-01, 3.7130000e-01, 6.4560000e-01, 1.0390501e+00,
        1.3856000e+00, 1.6229600e+00, 1.7470600e+00, 1.7826000e+00, 1.7721100e+00,
        1.7441000e+00, 1.6692000e+00, 1.5281000e+00, 1.2876400e+00, 1.0419000e+00,
        8.1295010e-01, 6.1620000e-01, 4.6518000e-01, 3.5330000e-01, 2.7200000e-01,
        2.1230000e-01, 1.5820000e-01, 1.1170000e-01, 7.8249990e-02, 5.7250010e-02,
        4.2160000e-02, 2.9840000e-02, 2.0300000e-02, 1.3400000e-02, 8.7499990e-03,
        5.7499990e-03, 3.9000000e-03, 2.7499990e-03, 2.1000000e-03, 1.8000000e-03,
        1.6500010e-03, 1.4000000e-03, 1.1000000e-03, 1.0000000e-03, 8.0000000e-04,
        6.0000000e-04, 3.4000000e-04, 2.4000000e-04, 1.9000000e-04, 1.0000000e-04,
        4.9999990e-05, 3.0000000e-05, 2.0000000e-05, 1.0000000e-05, 0.0000000e+00,
        0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00,
        0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00,
        0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00,
        0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00,
        0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00, 0.0000000e+00,
    ]);

    pub const CIE_NORM: Float = 1.0 / 106.8564135;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mult() {
        let rgb = RGB::from([0.5, 0.5, 0.5]);
        assert_eq!(RGB::from([1.0, 1.0, 1.0]), rgb * 2.0);

        let mut xyz = XYZ::from([0.25, 0.25, 0.25]);
        xyz *= 4.0;
        assert_eq!(XYZ::from([1.0, 1.0, 1.0]), xyz);
    }

    #[test]
    fn div() {
        let rgb = RGB::from([1.0, 1.0, 1.0]);
        assert_eq!(RGB::from([0.5, 0.5, 0.5]), rgb / 2.0);

        let mut xyz = XYZ::from([1.0, 1.0, 1.0]);
        xyz /= 4.0;
        assert_eq!(XYZ::from([0.25, 0.25, 0.25]), xyz);
    }

    #[test]
    fn type_system() {
        let xyz1 = XYZ::from([0.25, 0.5, 0.75]);
        let xyz2 = XYZ::from([0.25, 0.5, 0.75]);
        let rgb1 = RGB::from([0.25, 0.5, 0.75]);
        let rgb2 = RGB::from([0.25, 0.5, 0.75]);

        let _ = xyz1 + xyz2;
        let _ = rgb1 + rgb2;
        // let _ = rgb1 + xyz1;
    }
}
