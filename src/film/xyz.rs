use std::ops::{Add, AddAssign, Mul};

use crate::{spectrum::Sampled, Float};

use super::consts;

/// A CIE 1931 tristimulus value.
///
/// Internally all colorspace data is stored in this format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZ(Float, Float, Float);

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

pub struct XYZConvert {
    cie_x: Sampled,
    cie_y: Sampled,
    cie_z: Sampled,
    cie_norm: Float,
}

impl XYZConvert {
    pub fn new() -> Self {
        Self {
            cie_x: consts::CIE_X.into(),
            cie_y: consts::CIE_X.into(),
            cie_z: consts::CIE_X.into(),
            cie_norm: 1.0,
        }
    }

    pub fn sampled_to_xyz(&self, spec: &Sampled) -> XYZ {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for (i, val) in spec.iter().enumerate() {
            x += val * self.cie_x[i];
            y += val * self.cie_y[i];
            z += val * self.cie_z[i];
        }

        XYZ(x, y, z) * self.cie_norm
    }
}
