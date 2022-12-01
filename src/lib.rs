//! # Gremlin
//!
//! Gremlin is a ray tracer

pub mod camera;
pub mod color;
pub mod film;
pub mod geo;
pub mod integrator;
pub mod material;
pub mod metrics;
pub mod prelude;
pub mod scene;
pub mod shape;
pub mod spectrum;

use camera::Camera;
use color::Color;
use film::Film;
use integrator::Integrator;
use rayon::prelude::*;

// Typedef for what floating-point value to use.
//
// Using generics was fine and all, but once you start getting outside the
// foundational `geo` package, it becomes a bigger and bigger headache.
//
// * Lots of packages need to define large arrays of constants which is a pain
//   to do generically, even with `num-traits`.
// * It starts impacting everything that depends on it, including things like
//   shape definitions and whatnot. It becomes noise throughout the whole
//   program.
// * We almost never want to mix which type we use within the same process
//   anyway, so what's the point.
//
// Thus the compile-time flag.
// See: <https://users.rust-lang.org/t/generics-using-either-f32-or-f64/28647/3>

/// The floating-point format used throughout Gremlin.
///
/// The use [`f64`], compile without the `--features "f32"` flag.
#[cfg(feature = "f32")]
pub type Float = f32;

/// The floating-point format used throughout Gremlin.
///
/// To use [`f32`], compile with the `--features "f32"` flag.
#[cfg(not(feature = "f32"))]
pub type Float = f64;

pub fn render<CS, Li>(film: &mut Film<CS>, cam: &impl Camera, integrator: &impl Integrator<Li>)
where
    Color<CS>: From<Li> + Copy + Send,
    CS: Copy,
{
    film.par_pixel_iter_mut()
        .for_each_init(rand::thread_rng, |rng, (px, py, pixel)| {
            let ray = cam.ray(px, py, rng);
            let rad = integrator.radiance(&ray, rng);
        });
}
