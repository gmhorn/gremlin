use num::{Num, Float};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self{x, y, z}
    }

    pub fn splat(n: T) -> Self {
        Self{x: n, y: n, z: n}
    }
}
