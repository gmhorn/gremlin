use std::ops::{Add, Mul, Sub, Neg};
use approx::{RelativeEq, AbsDiffEq, UlpsEq};
use crate::Real;
use super::{Vector, Point, Unit};

/// A 4x4 square matrix.
///
/// Stored internally in row-major format. Generally speaking, these are used
/// to encode 3-dimensional transformations. Homogeneous coordinates are
/// assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix<R> {
    data: [[R; 4]; 4],
}

// Helper typedef to make inverting somewhat more pleasant.
type AugmentedMatrix<R> = [[R; 8]; 4];

impl<R: Real> Matrix<R> {

    /// Construct an identity matrix.
    #[rustfmt::skip]
    #[inline]
    pub fn identity() -> Self {
        let one = R::one();
        let zero = R::zero();

        Self::from([
            [one,  zero, zero, zero],
            [zero, one,  zero, zero],
            [zero, zero, one,  zero],
            [zero, zero, zero, one],
        ])
    }

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
    pub fn shift(v: Vector<R>) -> Self {
        let one = R::one();
        let zero = R::zero();

        Self::from([
            [one,  zero, zero, v.x],
            [zero, one,  zero, v.y],
            [zero, zero, one,  v.z],
            [zero, zero, zero, one],
        ])
    }

    /// Construct a matrix representing uniform scaling by the given magnitude.
    /// 
    /// See also [`Self::scale()`].
    #[inline]
    pub fn scale_uniform(n: R) -> Self {
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
    pub fn scale(x: R, y: R, z: R) -> Self {
        Self::from([
            [x,         R::zero(), R::zero(), R::zero()],
            [R::zero(), y,         R::zero(), R::zero()],
            [R::zero(), R::zero(), z,         R::zero()],
            [R::zero(), R::zero(), R::zero(), R::one()],
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
    pub fn rotate(theta: R, axis: Unit<R>) -> Self {
        // Covert angle to radians and axis to vector (so we can get components)
        let theta = theta.to_radians();
        let axis = Vector::from(axis);

        // Precompute some constants
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let one = R::one();
        let zero = R::zero();

        // Rotation of first basis vector
        let d00 = axis.x * axis.x + (one - axis.x * axis.x) * cos_theta;
        let d01 = axis.x * axis.y * (one - cos_theta) - axis.z * sin_theta;
        let d02 = axis.x * axis.z * (one - cos_theta) + axis.y * sin_theta;
        // Rotation of second basis vector
        let d10 = axis.y * axis.x * (one - cos_theta) + axis.z * sin_theta;
        let d11 = axis.y * axis.y + (one - axis.y * axis.y) * cos_theta;
        let d12 = axis.y * axis.z * (one - cos_theta) - axis.x * sin_theta;
        // Rotation of third basis vector
        let d20 = axis.z * axis.x * (one - cos_theta) - axis.y * sin_theta;
        let d21 = axis.z * axis.y * (one - cos_theta) + axis.x * sin_theta;
        let d22 = axis.z * axis.z + (one - axis.z * axis.z) * cos_theta;

        Self::from([
            [d00,  d01,  d02,  zero],
            [d10,  d11,  d12,  zero],
            [d20,  d21,  d22,  zero],
            [zero, zero, zero, one]
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
    pub fn look_at(from: Point<R>, to: Point<R>, up: Vector<R>) -> Self {
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
        let x_axis: [R; 3] = x_axis.into();
        let y_axis: [R; 3] = y_axis.into();
        let z_axis: [R; 3] = z_axis.into();

        Self::from([
            [x_axis[0], y_axis[0], z_axis[0], from.x],
            [x_axis[1], y_axis[1], z_axis[1], from.y],
            [x_axis[2], y_axis[2], z_axis[2], from.z],
            [R::zero(), R::zero(), R::zero(), R::one()],
        ])
    }

    /// Construct a matrix that is the transpose of this matrix.
    #[rustfmt::skip]
    #[inline]
    pub fn transpose(&self) -> Self {
        Self::from([
            [self.data[0][0], self.data[1][0], self.data[2][0], self.data[3][0]],
            [self.data[0][1], self.data[1][1], self.data[2][1], self.data[3][1]],
            [self.data[0][2], self.data[1][2], self.data[2][2], self.data[3][2]],
            [self.data[0][3], self.data[1][3], self.data[2][3], self.data[3][3]],
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
                aug[i][c] = R::zero();
                // Reduce all remaining elements in row
                for j in (c + 1)..8 {
                    aug[i][j] = aug[i][j] - f * aug[c][j]
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
                    aug[j][k] = aug[j][k] - f * aug[i][k];
                }
            }
        }
        
        // Inverse is right half of augmented matrix
        let mut data = [[R::zero(); 4]; 4];
        for (idx, row) in aug.iter().enumerate() {
            data[idx][..].copy_from_slice(&row[4..]);
        }

        Some(Self { data })
    }

    fn create_augmented(&self) -> AugmentedMatrix<R> {
        let mut augmented = [[R::zero(); 8]; 4];

        let ident = Self::identity();
        let lhs_rows = self.data.iter();
        let rhs_rows = ident.data.iter();
        
        for (idx, (lhs, rhs)) in lhs_rows.zip(rhs_rows).enumerate() {
            augmented[idx][..4].copy_from_slice(lhs);
            augmented[idx][4..].copy_from_slice(rhs);
        }

        augmented
    }

    fn find_pivot(pos: usize, mtx: &AugmentedMatrix<R>) -> Option<usize> {
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

impl<R: Real> Neg for Matrix<R> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut data = self.data;

        for row in &mut data {
            for val in row {
                *val = val.neg();
            }
        }

        Self::Output{ data }
    }
}

impl<R: Real> Add for Matrix<R> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data;

        for (r, row) in data.iter_mut().enumerate() {
            for (c, val) in row.iter_mut().enumerate() {
                *val += rhs.data[r][c];
            }
        }

        Self::Output { data }
    }
}

impl<R: Real> Sub for Matrix<R> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data;

        for (r, row) in data.iter_mut().enumerate() {
            for (c, val) in row.iter_mut().enumerate() {
                *val -= rhs.data[r][c];
            }
        }

        Self::Output { data }
    }
}

impl<R: Real> Mul for Matrix<R> {
    type Output = Self;

    // TODO: Not smart enough to figure out how to convert naive range looping
    // TODO: into iterative method. So just turn off the linter for now.
    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[R::zero(); 4]; 4];

        for r in 0..4 {
            for c in 0..4 {
                for k in 0..4 {
                    data[r][c] = data[r][c] + self.data[r][k] * rhs.data[k][c];
                }
            }
        }

        Self::Output { data }
    }
}

impl<R: Real> Mul<R> for Matrix<R> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: R) -> Self::Output {
        let mut data = self.data;

        for row in &mut data {
            for val in row {
                *val *= rhs;
            }
        }
        
        Self::Output { data }
    }
}

impl<R: Real> Mul<Vector<R>> for Matrix<R> {
    type Output = Vector<R>;

    #[inline]
    fn mul(self, rhs: Vector<R>) -> Self::Output {
        Self::Output {
            x: self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z,
            y: self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z,
            z: self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z,
        }
    }
}

impl<R: Real> Mul<Point<R>> for Matrix<R> {
    type Output = Point<R>;

    #[inline]
    fn mul(self, rhs: Point<R>) -> Self::Output {
        Self::Output {
            x: self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3],
            y: self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3],
            z: self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3],
        }
    }
    
}

// CONVERSIONS: OTHER -> MATRIX

impl<R: Real> From<[R; 16]> for Matrix<R> {
    fn from(vals: [R; 16]) -> Self {
        let mut data = [[R::zero(); 4]; 4];

        for (idx, &val) in vals.iter().enumerate() {
            let row = idx / 4;
            let col = idx % 4;
            data[row][col] = val;
        }

        Self::from(data)
    }
}

impl<R> From<[[R; 4]; 4]> for Matrix<R> {
    #[inline]
    fn from(data: [[R; 4]; 4]) -> Self {
        Self { data }
    }
}

// APPROXIMATIONS

impl<R: AbsDiffEq> AbsDiffEq for Matrix<R> where
    R::Epsilon: Copy,
{
    type Epsilon = R::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        R::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let self_vals = self.data.iter().flatten();
        let other_vals = other.data.iter().flatten();

        self_vals.zip(other_vals).all(|(a, b)| {
            R::abs_diff_eq(a, b, epsilon)
        })
    }
}

impl<R: RelativeEq> RelativeEq for Matrix<R> where
    R::Epsilon: Copy,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        R::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
        let self_vals = self.data.iter().flatten();
        let other_vals = other.data.iter().flatten();

        self_vals.zip(other_vals).all(|(a, b)| {
            R::relative_eq(a, b, epsilon, max_relative)
        })
    }
}

impl<R: UlpsEq> UlpsEq for Matrix<R> where
    R::Epsilon: Copy,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        R::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        let self_vals = self.data.iter().flatten();
        let other_vals = other.data.iter().flatten();

        self_vals.zip(other_vals).all(|(a, b)| {
            R::ulps_eq(a, b, epsilon, max_ulps)
        })
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn matrix_identity() {
        let m = Matrix::identity();
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

        assert_relative_eq!(Matrix::from([
            [0.174737, -0.694737, -0.48, 0.715789],
            [-0.212632, 0.652632, 0.56, -0.642105],
            [-0.0147368, 0.0947368, -0.08, 0.0842105],
             [0.176842, -0.136842,  -0.04, -0.0105263],
        ]), m_inv, max_relative = 1e-5);
    }
}
