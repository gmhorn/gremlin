use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg, DivAssign};

use num_traits::Float;

use super::{Vec3, Point3};

/// A 4x4 square matrix.
///
/// Stored internally in row-major format. Generally speaking, these are used
/// to encode 3-dimensional transformations. Homogeneous coordinates are
/// assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mtx4<F> {
    data: [[F; 4]; 4],
}

// Helper typedef to make inverting somewhat more pleasant.
type AugmentedMatrix<F> = [[F; 8]; 4];

impl<F: Float> Mtx4<F> {

    /// Construct an identity matrix.
    #[rustfmt::skip]
    #[inline]
    pub fn identity() -> Self {
        let one = F::one();
        let zero = F::zero();

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
    /// let s = Vec3::new(3.0, 4.0, 5.0);
    /// let v = Vec3::splat(1.0);
    /// let p = Point3::splat(1.0);
    /// 
    /// assert_eq!(Mtx4::shift(v) * v, v);
    /// assert_eq!(Mtx4::shift(v) * p, p + v);
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
    pub fn shift(v: Vec3<F>) -> Self {
        let one = F::one();
        let zero = F::zero();

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
    pub fn scale_uniform(n: F) -> Self {
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
    pub fn scale(x: F, y: F, z: F) -> Self {
        Self::from([
            [x,         F::zero(), F::zero(), F::zero()],
            [F::zero(), y,         F::zero(), F::zero()],
            [F::zero(), F::zero(), z,         F::zero()],
            [F::zero(), F::zero(), F::zero(), F::one()],
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
    pub fn rotate(theta: F, axis: Vec3<F>) -> Self {
        let theta = theta.to_radians();

        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let one = F::one();
        let zero = F::zero();

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
}

impl<F> Mtx4<F> where
    F: Float + SubAssign<F> + DivAssign<F>
{
    /// Construct a matrix that is the inverse of this matrix.
    /// 
    /// Uses Gauss-Jordan elimination to perform the inversion. See also:
    /// * <https://en.wikipedia.org/wiki/Gaussian_elimination>
    /// * <https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry>
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
            // Save off pivot value
            let pivot_val = aug[c][c];

            // For all rows below the pivot...
            for row in aug.iter_mut().skip(c+1) {
                let f = row[c] / pivot_val;
                // Fill the column below pivot with 0
                row[c] = F::zero();
                // Reduce all remaining elements in row
                for val in row.iter_mut().skip(c + 1) {
                    *val -= f * pivot_val;
                }
            }
        }

        // Back substitute
        for (i, row) in aug.iter_mut().enumerate().rev() {
            let f = row[i];
            for val in row.iter_mut() {
                *val /= f;
            }
        }

        let mut data = [[F::zero(); 4]; 4];
        for (idx, row) in aug.iter().enumerate() {
            data[idx][..].copy_from_slice(&row[4..]);
        }

        Some(Self { data })
    }

    fn create_augmented(&self) -> AugmentedMatrix<F> {
        let mut augmented = [[F::zero(); 8]; 4];

        let ident = Self::identity();
        let lhs_rows = self.data.iter();
        let rhs_rows = ident.data.iter();
        
        for (idx, (lhs, rhs)) in lhs_rows.zip(rhs_rows).enumerate() {
            augmented[idx][..4].copy_from_slice(lhs);
            augmented[idx][4..].copy_from_slice(rhs);
        }

        augmented
    }

    fn find_pivot(pos: usize, mtx: &AugmentedMatrix<F>) -> Option<usize> {
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

impl<F: Float> Neg for Mtx4<F> {
    type Output = Self;

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

impl<F: Float + AddAssign> Add for Mtx4<F> {
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

impl<F: Float + SubAssign> Sub for Mtx4<F> {
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

impl<F: Float + AddAssign> Mul for Mtx4<F> {
    type Output = Self;

    // TODO: Not smart enough to figure out how to convert naive range looping
    // TODO: into iterative method. So just turn off the linter for now.
    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut data = [[F::zero(); 4]; 4];

        for r in 0..4 {
            for c in 0..4 {
                for k in 0..4 {
                    data[r][c] += self.data[r][k] * rhs.data[k][c];
                }
            }
        }

        Self::Output { data }
    }
}

impl<F: Float + MulAssign> Mul<F> for Mtx4<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self::Output {
        let mut data = self.data;

        for row in &mut data {
            for val in row {
                *val *= rhs;
            }
        }
        
        Self::Output { data }
    }
}

impl<F: Float> Mul<Vec3<F>> for Mtx4<F> {
    type Output = Vec3<F>;

    fn mul(self, rhs: Vec3<F>) -> Self::Output {
        Self::Output {
            x: self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z,
            y: self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z,
            z: self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z,
        }
    }
}

impl<F: Float> Mul<Point3<F>> for Mtx4<F> {
    type Output = Point3<F>;

    fn mul(self, rhs: Point3<F>) -> Self::Output {
        Self::Output {
            x: self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3],
            y: self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3],
            z: self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3],
        }
    }
    
}

// CONVERSIONS

impl<F: Float> From<[F; 16]> for Mtx4<F> {
    fn from(vals: [F; 16]) -> Self {
        let mut data = [[F::zero(); 4]; 4];

        for (idx, &val) in vals.iter().enumerate() {
            let row = idx / 4;
            let col = idx % 4;
            data[row][col] = val;
        }

        Self::from(data)
    }
}

impl<F> From<[[F; 4]; 4]> for Mtx4<F> {
    #[inline]
    fn from(data: [[F; 4]; 4]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_identity() {
        let m = Mtx4::identity();
        let v = Vec3::new(1.0, 2.0, 3.0);
        let p = Point3::new(6.0, 7.0, 8.0);

        assert_eq!(m * v, v);
        assert_eq!(m * p, p);
    }

    #[test]
    fn matrix_shift() {
        let m = Mtx4::shift(Vec3::new(3.0, 4.0, 5.0));
        let v = Vec3::splat(1.0);
        let p = Point3::splat(1.0);

        assert_eq!(m * v, v);
        assert_eq!(m * p, Point3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn matrix_scale() {
        let m = Mtx4::scale(3.0, 4.0, 5.0);
        let v = Vec3::splat(1.0);
        let p = Point3::splat(1.0);

        assert_eq!(m * v, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(m * p, Point3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn matrix_add() {
        let m = Mtx4::scale_uniform(3.0);
        let n = Mtx4::scale_uniform(5.0);

        assert_eq!(
            m + n,
            Mtx4::from([
                [8.0, 0.0, 0.0, 0.0],
                [0.0, 8.0, 0.0, 0.0],
                [0.0, 0.0, 8.0, 0.0],
                [0.0, 0.0, 0.0, 2.0],
            ])
        );
    }

    #[test]
    fn matrix_inverse() {
        let m = Mtx4::from([
            [3.0, 4.0, 6.0, 8.0],
            [1.0, 2.0, 7.0, 2.0],
            [8.0, 9.0, 1.0, 3.0],
            [7.0, 7.0, 6.0, 2.0],
        ]);
        let m_inv = m.inverse();

        /*
          0.174737  -0.694737  -0.48 0.715789
         -0.212632   0.652632   0.56 -0.642105
         -0.0147368  0.0947368 -0.08 0.0842105
          0.176842  -0.136842  -0.04 -0.0105263
        */

        println!("{:?}", m_inv);
    }
}
