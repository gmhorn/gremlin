use crate::{camera::Camera, film::{Film, Buffer}};

pub trait Integrator {
    fn render<P>(cam: impl Camera, film: Film<P>) ->  Buffer<P>;
}