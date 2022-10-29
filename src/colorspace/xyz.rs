use std::ops::Mul;

use num_traits::Float;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct XYZ<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> XYZ<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<F: Float> Mul<F> for XYZ<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Self::Output::new(rhs * self.x, rhs * self.y, rhs * self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_min() {
        let scalar = 2.0;
        let xyz = XYZ::new(1.0, 2.0, 3.0);
        let _post_mult = xyz * scalar;
    }
}
