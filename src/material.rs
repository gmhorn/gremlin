use rand::Rng;

use crate::{color::RGB, geo::Ray, shape::Intersection};

mod lambertian;
pub use lambertian::*;

pub trait BSDF {
    fn scatter(&self, ray: &Ray, isec: &Intersection, rng: &mut impl Rng) -> Option<(RGB, Ray)>;
}

pub enum Material {}

#[derive(Debug)]
pub struct Mat {
    name: String,
}

impl Mat {
    pub fn new(name: String) -> Self {
        Self { name: name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatHandle<'a> {
    pub mat: &'a Mat,
}

pub struct MatStore(Vec<Mat>);

impl MatStore {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, mat: Mat) -> MatHandle {
        self.0.push(mat);
        MatHandle {
            mat: &self.0[self.0.len() - 1],
        }
    }
}

pub struct Prim<'a> {
    mat: MatHandle<'a>,
}

impl<'a> Prim<'a> {
    pub fn new(mat: MatHandle<'a>) -> Self {
        Self { mat: mat }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn store() {
        let mut store = MatStore::new();
        let mat1 = Mat::new(String::from("mat1"));
        let mat2 = Mat::new(String::from("mat2"));

        let s = Prim::new(store.add(mat1));
    }
}
