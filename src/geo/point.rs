use std::ops::{Add, Sub};
use super::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> Point3<T> {
    /// Creates a new point.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self{x, y, z}
    }

    /// Creates a point with all elements set to `n`.
    pub const fn splat(n: T) -> Self {
        Self{x: n, y: n, z: n}
    }
}

impl<T: Add<Output = T>> Add for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Add<Output = T>> Add<Vec3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
    
}