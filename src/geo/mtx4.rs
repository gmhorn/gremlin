use std::ops::{Add, AddAssign, Mul, Sub, MulAssign, SubAssign};

use num_traits::{Float, Num};

/// A 4x4 square matrix.
///
/// Stored internally in row-major format. Generally speaking, these are used
/// to encode 3-dimensional transformations. Homogeneous coordinates are
/// assumed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mtx4<F> {
    data: [[F; 4]; 4],
}

impl<F: Float> Mtx4<F> {
    /// Construct an identity matrix (`1` along the main diagonal, `0`
    /// everywhere else).
    #[inline]
    pub fn identity() -> Self {
        Self::from([
            [F::zero(), F::zero(), F::zero(), F::zero()],
            [F::zero(), F::one(), F::zero(), F::zero()],
            [F::zero(), F::zero(), F::one(), F::zero()],
            [F::zero(), F::zero(), F::zero(), F::one()],
        ])
    }

    #[rustfmt::skip]
    #[inline]
    pub fn scale(n: F) -> Self {
        Self::from([
            [n,         F::zero(), F::zero(), F::zero()],
            [F::zero(), n,         F::zero(), F::zero()],
            [F::zero(), F::zero(), n,         F::zero()],
            [F::zero(), F::zero(), F::zero(), F::one()],
        ])
    }

    /// Create a matrix that is the transpose of this matrix.
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

// OPERATORS

impl<F: Float + AddAssign> Add for Mtx4<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();

        for r in 0..4 {
            for c in 0..4 {
                data[r][c] += rhs.data[r][c];
            }
        }

        Self::Output { data }
    }
}

impl<F: Float + SubAssign> Sub for Mtx4<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data.clone();

        for r in 0..4 {
            for c in 0..4 {
                data[r][c] -= rhs.data[r][c];
            }
        }

        Self::Output { data }
    }
}

impl<F: Float + AddAssign> Mul for Mtx4<F> {
    type Output = Self;

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
        let mut data = self.data.clone();

        for r in 0..4 {
            for c in 0..4 {
                data[r][c] *= rhs;
            }
        }

        Self::Output{ data }
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
    fn matrix_add() {
        let m = Mtx4::scale(3.0);
        let n = Mtx4::scale(5.0);

        assert_eq!(m + n, Mtx4::from([
            [8.0, 0.0, 0.0, 0.0],
            [0.0, 8.0, 0.0, 0.0],
            [0.0, 0.0, 8.0, 0.0],
            [0.0, 0.0, 0.0, 2.0],
        ]));
    }
}