use super::{Point, Vector};

pub struct Ray {
    origin: Point,
    dir: Vector,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point, dir: Vector) -> Self {
        Self { origin, dir }
    }

    #[inline]
    pub fn origin(&self) -> Point {
        self.origin
    }

    #[inline]
    pub fn dir(&self) -> Vector {
        self.dir
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point {
        self.origin + (t * self.dir)
    }
}
