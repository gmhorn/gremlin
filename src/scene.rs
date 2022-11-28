use crate::{material::Material, shape::Surface};

pub struct Scene {}

impl Scene {
    pub fn add_primitive<S, M>(&mut self, _surface: S, _material: M)
    where
        Surface: From<S>,
        Material: From<M>,
    {
        todo!()
    }
}
