use super::{PointOld, Vector};

pub struct Ray {
    origin: PointOld,
    dir: Vector,
}

impl Ray {
    #[inline]
    pub fn new(origin: PointOld, dir: Vector) -> Self {
        Self { origin, dir }
    }

    #[inline]
    pub fn origin(&self) -> PointOld {
        self.origin
    }

    #[inline]
    pub fn dir(&self) -> Vector {
        self.dir
    }

    #[inline]
    pub fn at(&self, t: f64) -> PointOld {
        self.origin + (t * self.dir)
    }
}
