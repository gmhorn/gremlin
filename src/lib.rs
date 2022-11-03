//! # Gremlin
//!
//! Gremlin is a ray tracer
pub mod film;
pub mod geo;
pub mod spectrum;

/// The floating-point format used throughout Gremlin.
///
/// Defaults to [`f64`], but can be set to [`f32`] using `--features "f32"` in
/// Cargo
///
/// See: <https://users.rust-lang.org/t/generics-using-either-f32-or-f64/28647/3>
#[cfg(feature = "f32")]
pub type MyFloat = f32;
#[cfg(feature = "f32")]
pub use std::f32 as floats;
#[cfg(not(feature = "f32"))]
pub type MyFloat = f64;
#[cfg(not(feature = "f32"))]
pub use std::f64 as floats;
