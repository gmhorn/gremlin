use std::ops::{Add, Mul};

use super::{PointOld, RayOld, UnitOld, VectorOld};

/// A row-major, 4x4 "real-valued" (`f64`-valued) matrix.
///
/// Implicitly, all operations on points and vectors are
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatrixOld {
    data: [[f64; 4]; 4],
}

impl MatrixOld {
    pub const IDENTITY: Self = Self {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    /// Constructs a new matrix representing a shift by the given vector.
    ///
    /// Note that multiplication by a vector leaves the vector unchanged, while
    /// multiplication by a point translates the point
    ///
    /// ```
    /// use gremlin::geo::{Matrix, Point, Vector};
    /// let mtx = Matrix::shift(Vector::new(1.0, 2.0, 3.0));
    ///
    /// assert_eq!(&mtx * Vector::new(1.0, 1.0, 1.0), Vector::new(1.0, 1.0, 1.0));
    /// assert_eq!(&mtx * Point::new(1.0, 1.0, 1.0), Point::new(2.0, 3.0, 4.0));
    /// ```
    ///
    /// Note that the inverse of `shift(v)` is `shift(-v)`.
    ///
    /// See: https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Translations
    pub fn shift(v: VectorOld) -> Self {
        Self::from([
            [1.0, 0.0, 0.0, v.x],
            [0.0, 1.0, 0.0, v.y],
            [0.0, 0.0, 1.0, v.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Constructs a new matrix representing scaling by a given vector. The
    /// coordinates of the vector are the scale factor for each axis. So
    ///
    /// ```
    /// use gremlin::geo::{Matrix, Vector};
    /// let _mtx = Matrix::scale(Vector::new(1.0, 2.0, 0.0));
    /// ```
    ///
    /// creates a matrix that scales by 1 unit in the x-direction and 2 in the
    /// y-direction.
    ///
    /// Note that the inverse of `scale(v)` is `scale(w)` where the components
    /// of `w` are the reciprocals of the components of `v`.
    ///
    /// See: https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#Scaling
    pub fn scale(v: VectorOld) -> Self {
        Self::from([
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Constructs a new matrix representing rotation by an angle about the
    /// given axis.
    ///
    /// Note that the inverse of a rotation matrix is equal to its transpose.
    ///
    /// See: https://www.pbr-book.org/3ed-2018/Geometry_and_Transformations/Transformations#RotationaroundanArbitraryAxis
    pub fn rotate(theta: f64, axis: UnitOld) -> Self {
        let mut data = [[0.0; 4]; 4];

        let theta = theta.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        // Rotation of first basis vector
        data[0][0] = axis.x * axis.x + (1.0 - axis.x * axis.x) * cos_theta;
        data[0][1] = axis.x * axis.y * (1.0 - cos_theta) - axis.z * sin_theta;
        data[0][2] = axis.x * axis.z * (1.0 - cos_theta) + axis.y * sin_theta;
        // Rotation of second basis vector
        data[1][0] = axis.y * axis.x * (1.0 - cos_theta) + axis.z * sin_theta;
        data[1][1] = axis.y * axis.y + (1.0 - axis.y * axis.y) * cos_theta;
        data[1][2] = axis.y * axis.z * (1.0 - cos_theta) - axis.x * sin_theta;
        // Rotation of third basis vector
        data[2][0] = axis.z * axis.x * (1.0 - cos_theta) - axis.y * sin_theta;
        data[2][1] = axis.z * axis.y * (1.0 - cos_theta) + axis.x * sin_theta;
        data[2][2] = axis.z * axis.z + (1.0 - axis.z * axis.z) * cos_theta;
        // Final row identical to identity matrix
        data[3][3] = 1.0;
        Self { data }
    }

    /// Returns a view matrix that can translate from camera space to world
    /// space. All arguments are in world-space coordinates. `from` gives the
    /// camera location and `to` gives the
    pub fn look_at(from: PointOld, to: PointOld, up: VectorOld) -> Self {
        let z_axis = from - to;
        let x_axis = up.cross(z_axis);
        let y_axis = z_axis.cross(x_axis);

        let x_axis = x_axis
            .try_normalize()
            .expect("failed to construct orthonormal basis");
        let y_axis = y_axis
            .try_normalize()
            .expect("failed to construct orthonormal basis");
        let z_axis = z_axis
            .try_normalize()
            .expect("failed to construct orthonormal basis");

        Self::from([
            [x_axis.x, y_axis.x, z_axis.x, from.x],
            [x_axis.y, y_axis.y, z_axis.y, from.y],
            [x_axis.z, y_axis.z, z_axis.z, from.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a new matrix that is the transpose of this matrix.
    ///
    /// ```
    /// use gremlin::geo::*;
    /// let m = Matrix::look_at(Point::ORIGIN, Point::new(10.0, 10.0, 10.0), Vector::Y_AXIS);
    /// assert_eq!(m, m.transpose().transpose());
    /// ```
    pub fn transpose(&self) -> Self {
        let mut data = self.data.clone();

        for i in 0..4 {
            for j in 0..i {
                data.swap(i, j);
            }
        }

        Self { data }
    }

    /// Returns a new matrix that is the inverse of this matrix. Uses simple
    /// Gauss-Jordan elimination with partial pivoting.
    ///
    /// See:
    /// * https://en.wikipedia.org/wiki/Gaussian_elimination
    /// * https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/geometry
    pub fn inverse(&self) -> Self {
        // Create augmented matrix
        let mut m = [[0.0; 8]; 4];
        for i in 0..4 {
            m[i][..4].copy_from_slice(&self.data[i][..]);
            m[i][4..].copy_from_slice(&MatrixOld::IDENTITY.data[i][..]);
        }

        // Forward substitute
        let mut h = 0;
        let mut k = 0;
        while h < 4 && k < 8 {
            // Find k=th pivot
            let pivot = find_pivot(h, k, &m);
            // If no pivot in column, move to next column
            if m[pivot][k] == 0.0 {
                k += 1;
                continue;
            }
            // If pivot row not current row, swap rows
            if pivot != h {
                m.swap(pivot, h);
            }

            // For all rows below the pivot...
            for i in (h + 1)..4 {
                let f = m[i][k] / m[h][k];
                // Fill the rest of the column below pivot with 0
                m[i][k] = 0.0;
                // Reduce all remaining elements in row
                for j in (k + 1)..8 {
                    m[i][j] -= f * m[h][j];
                }
            }
            // increment pivot row and column
            h += 1;
            k += 1;
        }

        // Back substitute
        for i in (0..4).rev() {
            let f = m[i][i];
            for j in 0..8 {
                m[i][j] /= f;
            }

            for j in 0..i {
                let f = m[j][i];

                for k in 0..8 {
                    m[j][k] -= f * m[i][k];
                }
            }
        }

        // Inverse is right half of augmented matrix
        let mut data = [[0.0; 4]; 4];
        for i in 0..4 {
            data[i][..].copy_from_slice(&m[i][4..]);
        }
        Self { data }
    }
}

fn find_pivot(h: usize, k: usize, m: &[[f64; 8]; 4]) -> usize {
    let mut max = m[h][k].abs();
    let mut pivot = h;

    for i in (h + 1)..4 {
        if m[i][k].abs() > max {
            max = m[i][k].abs();
            pivot = i
        }
    }

    pivot
}

impl From<[[f64; 4]; 4]> for MatrixOld {
    #[inline]
    fn from(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }
}

impl From<[f64; 16]> for MatrixOld {
    fn from(coeffs: [f64; 16]) -> Self {
        let mut data = [[0.0; 4]; 4];

        for (idx, &val) in coeffs.iter().enumerate() {
            let row = idx / 4;
            let col = idx % 4;
            data[row][col] = val;
        }

        Self { data }
    }
}

impl Add<&MatrixOld> for &MatrixOld {
    type Output = MatrixOld;

    fn add(self, rhs: &MatrixOld) -> Self::Output {
        let mut data = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        Self::Output { data }
    }
}

impl Mul<&MatrixOld> for &MatrixOld {
    type Output = MatrixOld;

    fn mul(self, rhs: &MatrixOld) -> Self::Output {
        let mut data = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        Self::Output { data }
    }
}

impl<T: Into<VectorOld>> Mul<T> for &MatrixOld {
    type Output = VectorOld;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let a = self.data;
        Self::Output {
            x: a[0][0] * rhs.x + a[0][1] * rhs.y + a[0][2] * rhs.z,
            y: a[1][0] * rhs.x + a[1][1] * rhs.y + a[1][2] * rhs.z,
            z: a[2][0] * rhs.x + a[2][1] * rhs.y + a[2][2] * rhs.z,
        }
    }
}

impl Mul<PointOld> for &MatrixOld {
    type Output = PointOld;

    #[inline]
    fn mul(self, rhs: PointOld) -> Self::Output {
        let a = self.data;
        Self::Output {
            x: a[0][0] * rhs.x + a[0][1] * rhs.y + a[0][2] * rhs.z + a[0][3],
            y: a[1][0] * rhs.x + a[1][1] * rhs.y + a[1][2] * rhs.z + a[1][3],
            z: a[2][0] * rhs.x + a[2][1] * rhs.y + a[2][2] * rhs.z + a[2][3],
        }
    }
}

impl Mul<&RayOld> for &MatrixOld {
    type Output = RayOld;

    #[inline]
    fn mul(self, rhs: &RayOld) -> Self::Output {
        RayOld::new(self * rhs.origin(), self * rhs.dir())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_inverse() {
        let m = MatrixOld::from([
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
