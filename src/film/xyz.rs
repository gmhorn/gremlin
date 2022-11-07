use super::{Buffer, Save};
use crate::{
    geo::{Matrix, Vector},
    spectrum::Sampled,
    Float,
};
use image::{ImageResult, Rgb, RgbImage};
use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
    path::Path,
};

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

impl MulAssign<Float> for XYZ {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<Float> for XYZ {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Float) -> Self::Output {
        self * rhs.recip()
    }
}

impl DivAssign<Float> for XYZ {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        *self *= rhs.recip();
    }
}

// CONVERSIONS: XYZ -> OTHER

impl From<XYZ> for [Float; 3] {
    #[inline]
    fn from(xyz: XYZ) -> Self {
        [xyz.0, xyz.1, xyz.2]
    }
}

impl From<&XYZ> for Rgb<u8> {
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
    fn from(xyz: &XYZ) -> Self {
        // Work in vector space, since it already implements the operations we
        // need.
        let xyz = Vector::new(xyz.0, xyz.1, xyz.2);

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

// CONVERSIONS: OTHER -> XYZ

impl From<&Sampled> for XYZ {
    /// Converts a sampled spectrum to XYZ values by integrating against the
    /// CIE color-matching curves.
    #[inline]
    fn from(sampled: &Sampled) -> Self {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for (i, val) in sampled.iter().enumerate() {
            x += val * CIE_X[i];
            y += val * CIE_Y[i];
            z += val * CIE_Z[i];
        }

        XYZ(x, y, z) * CIE_NORM
    }
}

impl From<[Float; 3]> for XYZ {
    #[inline]
    fn from(arr: [Float; 3]) -> Self {
        Self(arr[0], arr[1], arr[2])
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
        1.055 * v.powf(0.41667) - 0.055
    }
}

// BUFFER IMPLEMENTATIONS

/// A buffer whose pixels are XYZ tristimulus values.
pub type XYZBuffer = Buffer<XYZ>;

impl Save for Buffer<XYZ> {
    /// Saves the buffer to a file at the path specified.
    ///
    /// The image format is derived from the file extension. Image is converted
    /// to sRGB color space.
    fn save_image<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>,
    {
        RgbImage::from_fn(self.width(), self.height(), |x, y| {
            self.get_pixel(x, y).into()
        })
        .save(path)
    }
}

const CIE_X: Sampled = Sampled::new([
    0.001368, 0.002236, 0.004243, 0.007650, 0.014310, 0.023190, 0.043510, 0.077630, 0.134380,
    0.214770, 0.283900, 0.328500, 0.348280, 0.348060, 0.336200, 0.318700, 0.290800, 0.251100,
    0.195360, 0.142100, 0.095640, 0.057950, 0.032010, 0.014700, 0.004900, 0.002400, 0.009300,
    0.029100, 0.063270, 0.109600, 0.165500, 0.225750, 0.290400, 0.359700, 0.433450, 0.512050,
    0.594500, 0.678400, 0.762100, 0.842500, 0.916300, 0.978600, 1.026300, 1.056700, 1.062200,
    1.045600, 1.002600, 0.938400, 0.854450, 0.751400, 0.642400, 0.541900, 0.447900, 0.360800,
    0.283500, 0.218700, 0.164900, 0.121200, 0.087400, 0.063600, 0.046770, 0.032900, 0.022700,
    0.015840, 0.011359, 0.008111, 0.005790, 0.004109, 0.002899, 0.002049, 0.001440, 0.001000,
    0.000690, 0.000476, 0.000332, 0.000235, 0.000166, 0.000117, 0.000083, 0.000059,
]);

const CIE_Y: Sampled = Sampled::new([
    0.000039, 0.000064, 0.000120, 0.000217, 0.000396, 0.000640, 0.001210, 0.002180, 0.004000,
    0.007300, 0.011600, 0.016840, 0.023000, 0.029800, 0.038000, 0.048000, 0.060000, 0.073900,
    0.090980, 0.112600, 0.139020, 0.169300, 0.208020, 0.258600, 0.323000, 0.407300, 0.503000,
    0.608200, 0.710000, 0.793200, 0.862000, 0.914850, 0.954000, 0.980300, 0.994950, 1.000000,
    0.995000, 0.978600, 0.952000, 0.915400, 0.870000, 0.816300, 0.757000, 0.694900, 0.631000,
    0.566800, 0.503000, 0.441200, 0.381000, 0.321000, 0.265000, 0.217000, 0.175000, 0.138200,
    0.107000, 0.081600, 0.061000, 0.044580, 0.032000, 0.023200, 0.017000, 0.011920, 0.008210,
    0.005723, 0.004102, 0.002929, 0.002091, 0.001484, 0.001047, 0.000740, 0.000520, 0.000361,
    0.000249, 0.000172, 0.000120, 0.000085, 0.000060, 0.000042, 0.000030, 0.000021,
]);

const CIE_Z: Sampled = Sampled::new([
    0.006450, 0.010550, 0.020050, 0.036210, 0.067850, 0.110200, 0.207400, 0.371300, 0.645600,
    1.039050, 1.385600, 1.622960, 1.747060, 1.782600, 1.772110, 1.744100, 1.669200, 1.528100,
    1.287640, 1.041900, 0.812950, 0.616200, 0.465180, 0.353300, 0.272000, 0.212300, 0.158200,
    0.111700, 0.078250, 0.057250, 0.042160, 0.029840, 0.020300, 0.013400, 0.008750, 0.005750,
    0.003900, 0.002750, 0.002100, 0.001800, 0.001650, 0.001400, 0.001100, 0.001000, 0.000800,
    0.000600, 0.000340, 0.000240, 0.000190, 0.000100, 0.000050, 0.000030, 0.000020, 0.000010,
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
    0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000, 0.000000,
]);

const CIE_NORM: Float = 1.0;
