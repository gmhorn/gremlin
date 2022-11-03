use crate::{spectrum::Sampled, Float};

pub struct XYZ {
    pub X: Float,
    pub Y: Float,
    pub Z: Float,
}

impl XYZ {
    pub fn from_sampled(s: Sampled) -> Self {
        todo!()
    }
}
