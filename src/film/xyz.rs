use crate::{spectrum::Sampled, MyFloat};

pub struct XYZ {
    pub X: MyFloat,
    pub Y: MyFloat,
    pub Z: MyFloat,
}

impl XYZ {
    pub fn from_sampled(s: Sampled<MyFloat>) -> Self {
        todo!()
    }
}