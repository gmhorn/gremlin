use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn splat(n: f64) -> Self {
        Self { x: n, y: n, z: n }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}
