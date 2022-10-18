use std::ops::Mul;

use super::{Point, Ray, Vector};

/// A row-major, 4x4 "real-valued" (`f64`-valued) matrix.
/// 
/// Implicitly, all operations on points and vectors are 
pub struct Matrix {
    data: [[f64; 4]; 4],
}

impl Matrix {
    pub fn new(data: [[f64; 4]; 4]) -> Matrix {
        Matrix { data }
    }

    pub fn from_array(coeffs: [f64; 16]) -> Matrix {
        let mut data = [[0.0; 4]; 4];

        for (idx, &val) in coeffs.iter().enumerate() {
            let row = idx / 4;
            let col = idx % 4;
            data[row][col] = val;
        }

        Matrix { data }
    }

    /// Constructs a new matrix representing a shift by the given vector.
    /// 
    /// 
    pub fn shift(v: Vector) -> Matrix {
        Matrix {
            data: [
                [1.0, 0.0, 0.0, v.x],
                [0.0, 1.0, 0.0, v.y],
                [0.0, 0.0, 1.0, v.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn scale(v: Vector) -> Matrix {
        Matrix {
            data: [
                [v.x, 0.0, 0.0, 0.0],
                [0.0, v.y, 0.0, 0.0],
                [0.0, 0.0, v.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let mut m = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }

        Matrix { data: m }
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
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

    fn mul(self, rhs: &Ray) -> Self::Output {
        Ray::new(self * rhs.origin(), self * rhs.dir())
    }
}
