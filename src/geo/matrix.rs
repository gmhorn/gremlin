use std::ops::{Mul, Add};

use super::{Point, Ray, Vector};

/// A row-major, 4x4 "real-valued" (`f64`-valued) matrix.
///
/// Implicitly, all operations on points and vectors are
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    data: [[f64; 4]; 4],
}

impl Matrix {
    pub const IDENTITY: Self = Self {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    /// Constructs a new matrix representing a shift by the given vector.
    pub fn shift(v: Vector) -> Self {
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
    pub fn scale(v: Vector) -> Self {
        Self::from([
            [v.x, 0.0, 0.0, 0.0],
            [0.0, v.y, 0.0, 0.0],
            [0.0, 0.0, v.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Returns a view matrix that can translate from camera space to world
    /// space. All arguments are in world-space coordinates. `from` gives the
    /// camera location and `to` gives the
    pub fn look_at(from: Point, to: Point, up: Vector) -> Self {
        let z_axis = from - to;
        let x_axis = up.cross(z_axis);
        let y_axis = z_axis.cross(x_axis);

        let x_axis = x_axis
            .normalize()
            .expect("failed to construct orthonormal basis");
        let y_axis = y_axis
            .normalize()
            .expect("failed to construct orthonormal basis");
        let z_axis = z_axis
            .normalize()
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

        Self{ data }
    }
}

impl From<[[f64; 4]; 4]> for Matrix {
    #[inline]
    fn from(data: [[f64; 4]; 4]) -> Self {
        Self { data }
    }
}

impl From<[f64; 16]> for Matrix {
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

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        let mut data = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }

        Self::Output{ data }
    }

    
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut data = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        Matrix { data }
    }
}

impl<T: Into<Vector>> Mul<T> for &Matrix {
    type Output = Vector;

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

impl Mul<Point> for &Matrix {
    type Output = Point;

    #[inline]
    fn mul(self, rhs: Point) -> Self::Output {
        let a = self.data;
        Self::Output {
            x: a[0][0] * rhs.x + a[0][1] * rhs.y + a[0][2] * rhs.z + a[0][3],
            y: a[1][0] * rhs.x + a[1][1] * rhs.y + a[1][2] * rhs.z + a[1][3],
            z: a[2][0] * rhs.x + a[2][1] * rhs.y + a[2][2] * rhs.z + a[2][3],
        }
    }
}

impl Mul<&Ray> for &Matrix {
    type Output = Ray;

    #[inline]
    fn mul(self, rhs: &Ray) -> Self::Output {
        Ray::new(self * rhs.origin(), self * rhs.dir())
    }
}
