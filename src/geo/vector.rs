#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn splat(n: f64) -> Vec3 {
        Vec3{x: n, y: n, z: n}
    }
}