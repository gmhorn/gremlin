use crate::{shape::Surface, material::Material};

pub struct Scene {}

impl Scene {
    pub fn add_primitive<S, M>(&mut self, surface: S, material: M)
    where
        Surface: From<S>,
        Material: From<M>,
    {
        todo!()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {  }
    }
}