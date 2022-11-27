use crate::{
    camera::Camera,
    color::Color,
    film::Film,
    geo::{Point, Ray, Vector},
};

pub trait Integrator<Li>: Send + Sync {
    fn radiance(&self, ray: &Ray) -> Li;
}

pub struct PathTracer {
    // immutable things, so tracer can be send + sync
    // scene
    // lights
    // camera
}

pub fn render<CS, Li>(film: &mut Film<CS>, cam: &impl Camera, integrator: &impl Integrator<Li>)
where
    Color<CS>: From<Li> + Copy + Send,
    CS: Copy,
{
    for _ in 0..1024 {
        film.add_samples(|x, y| {
            let ray = Ray::new(Point::ORIGIN, Vector::X_AXIS);
            integrator.radiance(&ray)
        });
    }
}
