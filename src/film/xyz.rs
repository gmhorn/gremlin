use std::{ops::{Add, AddAssign, Mul}, path::Path};
use image::{ImageResult, RgbImage, Rgb};
use crate::{spectrum::{Sampled, Conversion}, Float, geo::{Matrix, Vector}};
use super::Buffer;

/// A CIE 1931 tristimulus value.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct XYZ(Float, Float, Float);

impl XYZ {
    /// Create a new XYZ value.
    #[inline]
    pub const fn new(x: Float, y: Float, z: Float) -> Self {
        Self(x, y, z)
    }
}

impl Add for XYZ {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for XYZ {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Mul<Float> for XYZ {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Float) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

// CONVERSIONS: XYZ -> OTHER

impl From<XYZ> for [Float; 3] {
    #[inline]
    fn from(xyz: XYZ) -> Self {
        [xyz.0, xyz.1, xyz.2]
    }
}

impl From<XYZ> for Rgb<u8> {
    /// Creates a [`u8`]-valued RGB value from this XYZ value.
    /// 
    /// The `image` crate expects values to be in sRGB color space, so that is
    /// done as part of this conversion.
    /// 
    /// Internally, this first multiplies by a linear transformation to bring
    /// the XYZ value into linear RGB space. Then gamma correction is applied.
    /// If the result is outside gamut, desaturate by adding white (equal parts
    /// r, g, and b). Finally, if values are outside of range, clamp into range
    /// by uniformly scaling.
    /// 
    /// This is more-or-less a direct port of John Walker's code from his
    /// _Colour Rendering of Spectra_ page:
    /// * <https://www.fourmilab.ch/documents/specrend/>
    /// * <https://www.fourmilab.ch/documents/specrend/specrend.c>
    fn from(xyz: XYZ) -> Self {
        // Work in vector space, since it already implements the operations we
        // need. 
        let xyz: Vector = xyz.into();

        // Convert to linear RGB (matrix mult) then to sRGB (gamma)
        let mut rgb: Vector = (XYZ_TO_RGB * xyz).apply(gamma);

        // If out of gamut, desaturate
        let min = rgb.min_component();
        if min < 0.0 {
            rgb -= Vector::splat(min);
        }
        
        // Clamp max value
        let max = rgb.max_component();
        if max > 1.0 {
            rgb /= max;
        }

        // Scale by 255
        rgb *= 255.5;

        Rgb::from([rgb.x as u8, rgb.y as u8, rgb.z as u8])
    }
}

#[doc(hidden)]
impl From<XYZ> for Vector {
    #[inline]
    fn from(xyz: XYZ) -> Self {
        Self::new(xyz.0, xyz.1, xyz.2)
    }
}

// CONVERSIONS: OTHER -> XYZ

impl From<[Float; 3]> for XYZ {
    #[inline]
    fn from(arr: [Float; 3]) -> Self {
        Self(arr[0], arr[1], arr[2])
    }
}

#[doc(hidden)]
impl From<Vector> for XYZ {
    #[inline]
    fn from(v: Vector) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}



// Matrix for taking XYZ to linear RGB
//
// Values from Bruce Lindbloom's page
// http://www.brucelindbloom.com/
#[rustfmt::skip]
const XYZ_TO_RGB: Matrix = Matrix::new([
    [ 3.2404542, -1.5371385, -0.4985314, 0.0],
    [-0.9692660,  1.8760108,  0.0415560, 0.0],
    [ 0.0556434, -0.2040259,  1.0572252, 0.0],
    [ 0.0,        0.0,        0.0,       0.0]
]);

// Function for taking linear RGB to sRGB.
//
// Values from Bruce Lindbloom's page
// http://www.brucelindbloom.com/
fn gamma(v: Float) -> Float {
    if v <= 0.0031308 {
        12.92 * v 
    } else {
        1.055*v.powf(0.41667) - 0.055
    }
}

// BUFFER IMPLEMENTATIONS

/// A buffer whose pixels are XYZ tristimulus values.
pub type XYZBuffer = Buffer<XYZ>;

impl Buffer<XYZ> {
    /// Saves the buffer to a file at the path specified.
    /// 
    /// The image format is derived from the file extension. Image is converted
    /// to sRGB color space.
    pub fn save<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>
    {
        RgbImage::from_fn(self.width(), self.height(), |x, y| {
            (*self.get_pixel(x, y)).into()
        }).save(path)
    }
}

#[derive(Debug)]
pub struct ColorMatchingCurves {
    cie_x: Sampled,
    cie_y: Sampled,
    cie_z: Sampled,
    cie_norm: Float,
}

impl Conversion for ColorMatchingCurves {
    type Target = XYZ;

    fn convert(&self, sampled: Sampled) -> Self::Target {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for (i, val) in sampled.iter().enumerate() {
            x += val * self.cie_x[i];
            y += val * self.cie_y[i];
            z += val * self.cie_z[i];
        }

        XYZ(x, y, z) * self.cie_norm
    }
}
