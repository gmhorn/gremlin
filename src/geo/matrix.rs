use crate::MyFloat;

use super::{Point, Unit, Vector};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::{Add, Mul, Neg, Sub};

/// A 4x4 square matrix.
///
/// Stored internally in row-major format. Generally speaking, these are used
/// to encode 3-dimensional transformations. Homogeneous coordinates are
/// assumed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix([[MyFloat; 4]; 4]);

// Helper typedef to make inverting somewhat more pleasant.
type AugmentedMatrix = [[MyFloat; 8]; 4];

impl Matrix {

    /// The identity matrix.
    pub const IDENTITY: Matrix = Self([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    /// Construct a matrix representing translation by the given vector.
    ///
    /// Acts like an identity on vectors and addition on points.
    ///
    /// ```
    /// use gremlin::geo::*;
    ///
    /// let s = Vector::new(3.0, 4.0, 5.0);
    /// let v = Vector::splat(1.0);
    /// let p = Point::splat(1.0);
    ///
    /// assert_eq!(Matrix::shift(s) * v, v);
    /// assert_eq!(Matrix::shift(s) * p, p + s);
    /// ```
    ///
    /// Note that for inverses, it is much faster to use the identity:
    ///
    /// ```text
    /// shift(v).inverse() == shift(-v)
    /// ```
    ///
    /// See: <https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Translations>
    #[rustfmt::skip]
    #[inline]
    pub fn shift(v: Vector) -> Self {
        Self::from([
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Construct a matrix representing uniform scaling by the given magnitude.
    ///
    /// See also [`Self::scale()`].
    #[inline]
    pub fn scale_uniform(n: MyFloat) -> Self {
        Self::scale(n, n, n)
    }

    /// Construct a matrix representing scaling by the given magnitudes.
    ///
    /// Note that for inverses, it is much faster to use the identity:
    ///
    /// ```text
    /// scale(x, y, z).inverse() == scale(x.recip(), y.recip(), z.recip())
    /// ```
    ///
    /// See: <https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Scaling>
    #[rustfmt::skip]
    #[inline]
    pub fn scale(x: MyFloat, y: MyFloat, z: MyFloat) -> Self {
        Self::from([
            [  x, 0.0, 0.0, 0.0],
            [0.0,   y, 0.0, 0.0],
            [0.0, 0.0,   z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Construct a matrix representing rotation about the given axis.
    ///
    /// Assumes `theta` is given in degrees and internally converts to radians.
    ///
    /// Note that for inverses, it is much faster to use the identity:
    ///
    /// ```text
    /// rotate(theta, axis).transpose() == rotate(theta, axis).inverse()
    /// ```
    ///
    /// See: <https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#RotationaroundanArbitraryAxis>
    #[rustfmt::skip]
    pub fn rotate(theta: MyFloat, axis: Unit) -> Self {
        // Covert angle to radians and axis to vector (so we can get components)
        let theta = theta.to_radians();
        let axis = Vector::from(axis);

        // Precompute some constants
        let (sin_theta, cos_theta) = theta.sin_cos();

        // Rotation of first basis vector
        let d00 = axis.x * axis.x + (1.0 - axis.x * axis.x) * cos_theta;
        let d01 = axis.x * axis.y * (1.0 - cos_theta) - axis.z * sin_theta;
        let d02 = axis.x * axis.z * (1.0 - cos_theta) + axis.y * sin_theta;
        // Rotation of second basis vector
        let d10 = axis.y * axis.x * (1.0 - cos_theta) + axis.z * sin_theta;
        let d11 = axis.y * axis.y + (1.0 - axis.y * axis.y) * cos_theta;
        let d12 = axis.y * axis.z * (1.0 - cos_theta) - axis.x * sin_theta;
        // Rotation of third basis vector
        let d20 = axis.z * axis.x * (1.0 - cos_theta) - axis.y * sin_theta;
        let d21 = axis.z * axis.y * (1.0 - cos_theta) + axis.x * sin_theta;
        let d22 = axis.z * axis.z + (1.0 - axis.z * axis.z) * cos_theta;

        Self::from([
            [d00, d01, d02, 0.0],
            [d10, d11, d12, 0.0],
            [d20, d21, d22, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }

    /// Construct a right-handed look-at matrix.
    ///
    /// Useful for transforming camera space to world space. Conceptually:
    /// * `from` is the camera's location (world-space)
    /// * `to` is the point the camera's looking at (world space)
    /// * `up` is the vertical direction "according to the camera" (camera space)
    ///
    /// Using [`Vector::y_axis()`] will give a camera that's "pointing-up".
    ///
    /// See:
    /// * <https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#TheLook-AtTransformation>
    /// * <https://raytracing.github.io/books/RayTracingInOneWeekend.html#positionablecamera>
    pub fn look_at(from: Point, to: Point, up: Vector) -> Self {
        // Construct orthoginal basis
        let z_axis = from - to;
        let x_axis = up.cross(z_axis);
        let y_axis = z_axis.cross(x_axis);

        // Convert to orthonormal basis
        let x_axis = Unit::try_from(x_axis).expect("Failed to construct orthonormal basis");
        let y_axis = Unit::try_from(y_axis).expect("Failed to construct orthonormal basis");
        let z_axis = Unit::try_from(z_axis).expect("Failed to construct orthonormal basis");

        // Convert to array so we can grab elements
        // TODO: this kind of sucks...
        let x_axis: [MyFloat; 3] = x_axis.into();
        let y_axis: [MyFloat; 3] = y_axis.into();
        let z_axis: [MyFloat; 3] = z_axis.into();

        Self::from([
            [x_axis[0], y_axis[0], z_axis[0], from.x],
            [x_axis[1], y_axis[1], z_axis[1], from.y],
            [x_axis[2], y_axis[2], z_axis[2], from.z],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    /// Construct a matrix that is the transpose of this matrix.
    #[rustfmt::skip]
    #[inline]
    pub fn transpose(&self) -> Self {
        Self::from([
            [self.0[0][0], self.0[1][0], self.0[2][0], self.0[3][0]],
            [self.0[0][1], self.0[1][1], self.0[2][1], self.0[3][1]],
            [self.0[0][2], self.0[1][2], self.0[2][2], self.0[3][2]],
            [self.0[0][3], self.0[1][3], self.0[2][3], self.0[3][3]],
        ])
    }

    /// Construct a matrix that is the inverse of this matrix.
    ///
    /// Uses Gauss-Jordan elimination to perform the inversion. See also:
    /// * <https://en.wikipedia.org/wiki/Gaussian_elimination>
    /// * <https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry>
    // TODO: Not smart enough to figure out how to convert naive range looping
    // TODO: into iterative method. So just turn off the linter for now.
    #[allow(clippy::needless_range_loop)]
    pub fn inverse(&self) -> Option<Self> {
        let mut aug = self.create_augmented();

        // Forward substitute
        for c in 0..4 {
            // Find pivot for the current column
            let pivot = Self::find_pivot(c, &aug)?;
            // If pivot not current row, swap row
            if pivot != c {
                aug.swap(pivot, c);
            }

            // For all rows below the pivot...
            for i in (c + 1)..4 {
                let f = aug[i][c] / aug[c][c];
                // Fill the rest of the column below pivot with 0
                aug[i][c] = 0.0;
                // Reduce all remaining elements in row
                for j in (c + 1)..8 {
                    aug[i][j] -= f * aug[c][j]
                }
            }
        }

        // Back substitute
        for i in (0..4).rev() {
            let f = aug[i][i];
            for j in 0..8 {
                aug[i][j] = aug[i][j] / f;
            }

            for j in 0..i {
                let f = aug[j][i];

                for k in 0..8 {
                    aug[j][k] -= f * aug[i][k];
                }
            }
        }

        // Inverse is right half of augmented matrix
        let mut data = [[0.0; 4]; 4];
        for (idx, row) in aug.iter().enumerate() {
            data[idx][..].copy_from_slice(&row[4..]);
        }

        Some(Self::from(data))
    }

    fn create_augmented(&self) -> AugmentedMatrix {
        let mut augmented = [[0.0; 8]; 4];

        let ident = Self::IDENTITY;
        let lhs_rows = self.0.iter();
        let rhs_rows = ident.0.iter();

        for (idx, (lhs, rhs)) in lhs_rows.zip(rhs_rows).enumerate() {
            augmented[idx][..4].copy_from_slice(lhs);
            augmented[idx][4..].copy_from_slice(rhs);
        }

        augmented
    }

    fn find_pivot(pos: usize, mtx: &AugmentedMatrix) -> Option<usize> {
        let mut max = mtx[pos][pos].abs();
        let mut pivot = pos;

        for (i, row) in mtx.iter().enumerate().skip(pos + 1) {
            if row[pos].abs() > max {
                max = row[pos].abs();
                pivot = i;
            }
        }

        match max.abs().is_normal() {
            true => Some(pivot),
            false => None,
        }
    }
}

// OPERATORS

impl Neg for Matrix {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut data = self.0;

        for row in &mut data {
            for val in row {
                *val = val.neg();
            }
        }

        Self::Output::from(data)
    }
}

impl Add for Matrix {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.0;

        for (r, row) in data.iter_mut().enumerate() {
            for (c, val) in row.iter_mut().enumerate() {
                *val += rhs.0[r][c];
            }
        }

        Self::Output::from(data)
    }
}

impl Sub for Matrix {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.0;

        for (r, row) in data.iter_mut().enumerate() {
            for (c, val) in row.iter_mut().enumerate() {
                *val -= rhs.0[r][c];
            }
        }

        Self::Output::from(data)
    }
}

impl Mul for Matrix {
    type Output = Self;

    // TODO: Not smart enough to figure out how to convert naive range looping
    // TODO: into iterative method. So just turn off the linter for now.
    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[0.0; 4]; 4];

        for r in 0..4 {
            for c in 0..4 {
                for k in 0..4 {
                    data[r][c] += self.0[r][k] * rhs.0[k][c];
                }
            }
        }

        Self::Output::from(data)
    }
}

impl Mul<MyFloat> for Matrix {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: MyFloat) -> Self::Output {
        let mut data = self.0;

        for row in &mut data {
            for val in row {
                *val *= rhs;
            }
        }

        Self::Output::from(data)
    }
}

impl Mul<Matrix> for MyFloat {
    type Output = Matrix;

    #[inline]
    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.0[0][0] * rhs.x + self.0[0][1] * rhs.y + self.0[0][2] * rhs.z,
            y: self.0[1][0] * rhs.x + self.0[1][1] * rhs.y + self.0[1][2] * rhs.z,
            z: self.0[2][0] * rhs.x + self.0[2][1] * rhs.y + self.0[2][2] * rhs.z,
        }
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    #[rustfmt::skip]
    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.0[0][0] * rhs.x + self.0[0][1] * rhs.y + self.0[0][2] * rhs.z + self.0[0][3],
            y: self.0[1][0] * rhs.x + self.0[1][1] * rhs.y + self.0[1][2] * rhs.z + self.0[1][3],
            z: self.0[2][0] * rhs.x + self.0[2][1] * rhs.y + self.0[2][2] * rhs.z + self.0[2][3],
        }
    }
}

// CONVERSIONS: OTHER -> MATRIX

impl From<[MyFloat; 16]> for Matrix {
    fn from(vals: [MyFloat; 16]) -> Self {
        let mut data = [[0.0; 4]; 4];

        for (idx, &val) in vals.iter().enumerate() {
            let row = idx / 4;
            let col = idx % 4;
            data[row][col] = val;
        }

        Self::from(data)
    }
}

impl From<[[MyFloat; 4]; 4]> for Matrix {
    #[inline]
    fn from(data: [[MyFloat; 4]; 4]) -> Self {
        Self(data)
    }
}

// APPROXIMATIONS

impl AbsDiffEq for Matrix {
    type Epsilon = MyFloat;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        MyFloat::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let self_vals = self.0.iter().flatten();
        let other_vals = other.0.iter().flatten();

        self_vals
            .zip(other_vals)
            .all(|(a, b)| MyFloat::abs_diff_eq(a, b, epsilon))
    }
}

impl RelativeEq for Matrix {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        MyFloat::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        let self_vals = self.0.iter().flatten();
        let other_vals = other.0.iter().flatten();

        self_vals
            .zip(other_vals)
            .all(|(a, b)| MyFloat::relative_eq(a, b, epsilon, max_relative))
    }
}

impl UlpsEq for Matrix {
    #[inline]
    fn default_max_ulps() -> u32 {
        MyFloat::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let self_vals = self.0.iter().flatten();
        let other_vals = other.0.iter().flatten();

        self_vals
            .zip(other_vals)
            .all(|(a, b)| MyFloat::ulps_eq(a, b, epsilon, max_ulps))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn matrix_identity() {
        let m = Matrix::IDENTITY;
        let v = Vector::new(1.0, 2.0, 3.0);
        let p = Point::new(6.0, 7.0, 8.0);

        assert_eq!(m * v, v);
        assert_eq!(m * p, p);
    }

    #[test]
    fn matrix_shift() {
        let m = Matrix::shift(Vector::new(3.0, 4.0, 5.0));
        let v = Vector::splat(1.0);
        let p = Point::splat(1.0);

        assert_eq!(m * v, v);
        assert_eq!(m * p, Point::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn matrix_scale() {
        let m = Matrix::scale(3.0, 4.0, 5.0);
        let v = Vector::splat(1.0);
        let p = Point::splat(1.0);

        assert_eq!(m * v, Vector::new(3.0, 4.0, 5.0));
        assert_eq!(m * p, Point::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn matrix_add() {
        let m = Matrix::scale_uniform(3.0);
        let n = Matrix::scale_uniform(5.0);

        assert_eq!(
            m + n,
            Matrix::from([
                [8.0, 0.0, 0.0, 0.0],
                [0.0, 8.0, 0.0, 0.0],
                [0.0, 0.0, 8.0, 0.0],
                [0.0, 0.0, 0.0, 2.0],
            ])
        );
    }

    #[test]
    fn matrix_inverse() {
        let m = Matrix::from([
            [3.0, 4.0, 6.0, 8.0],
            [1.0, 2.0, 7.0, 2.0],
            [8.0, 9.0, 1.0, 3.0],
            [7.0, 7.0, 6.0, 2.0],
        ]);
        let m_inv = m.inverse().unwrap();

        assert_relative_eq!(
            Matrix::from([
                [0.174737, -0.694737, -0.48, 0.715789],
                [-0.212632, 0.652632, 0.56, -0.642105],
                [-0.0147368, 0.0947368, -0.08, 0.0842105],
                [0.176842, -0.136842, -0.04, -0.0105263],
            ]),
            m_inv,
            max_relative = 1e-5
        );
    }
}
