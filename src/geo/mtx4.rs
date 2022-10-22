use num_traits::Float;

use super::{Vec4, Vec3};

/// A 4x4 square matrix.
/// 
/// Stored internally in column-major format. Generally speaking, these are used
/// to encode 3-dimensional transformations. Homogeneous coordinates are
/// assumed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mtx4<F> {
    x_axis: Vec4<F>, // first column
    y_axis: Vec4<F>, // second column
    z_axis: Vec4<F>, // third column
    w_axis: Vec4<F>, // fourth column
}

impl<F: Float> Mtx4<F> {
    #[inline]
    const fn from_cols(cols: [Vec4<F>; 4]) -> Self {
        Self {
            x_axis: cols[0],
            y_axis: cols[1],
            z_axis: cols[2],
            w_axis: cols[3],
        }
    }

    /// Construct an identity matrix (`1` along the main diagonal, `0` 
    /// everywhere else).
    #[inline]
    pub fn identity() -> Self {
        Self::from_cols([Vec4::x_axis(), Vec4::y_axis(), Vec4::z_axis(), Vec4::w_axis()])
    }

    #[inline]
    pub fn scale(v: Vec3<F>) -> Self {
        Self::from_cols([
            Vec4::x_axis() * v.x,
            Vec4::y_axis() * v.y,
            Vec4::z_axis() * v.z,
            Vec4::w_axis()
        ])
    }
}

// CONVERSIONS

impl<F: Float> From<[F; 16]> for Mtx4<F> {
    fn from(_: [F; 16]) -> Self {
        todo!()
    }
}