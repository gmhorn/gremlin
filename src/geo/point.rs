#[derive(Debug)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn splat(n: f64) -> Point3 {
        Point3{x: n, y: n, z: n}
    }
}