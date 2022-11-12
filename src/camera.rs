use crate::{Float, geo::Ray};

mod perspective;
pub use perspective::*;

mod thin_lense;
pub use thin_lense::*;

pub trait Camera {
    fn ray(&self, u: Float, v: Float) -> Ray;
}