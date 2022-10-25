use super::{PointOld, VectorOld};

pub struct Ray {
    origin: PointOld,
    dir: VectorOld,
}

impl Ray {
    #[inline]
    pub fn new(origin: PointOld, dir: VectorOld) -> Self {
        Self { origin, dir }
    }

    #[inline]
    pub fn origin(&self) -> PointOld {
        self.origin
    }

    #[inline]
    pub fn dir(&self) -> VectorOld {
        self.dir
    }

    #[inline]
    pub fn at(&self, t: f64) -> PointOld {
        self.origin + (t * self.dir)
    }
}
