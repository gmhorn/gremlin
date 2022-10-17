use super::{Point, Vector};

pub struct Ray {
    origin: Point,
    dir: Vector,
}

impl Ray {
    pub fn new(origin: Point, dir: Vector) -> Self {
        Self { origin, dir }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn dir(&self) -> Vector {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + (t * self.dir)
    }
}
