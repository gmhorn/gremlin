//! # Gremlin
//!
//! Gremlin is a ray tracer

use std::ops::{AddAssign, MulAssign, SubAssign};

use num_traits::{Float, FromPrimitive};

pub mod film;
pub mod geo;
pub mod spectrum;

/// Trait defining "real-valued" numbers.
///
/// Allows us to be generic over both `f32` and `f64` primitive types.
pub trait Real:
    Float + 
    AddAssign +
    SubAssign +
    MulAssign +
    From<f32>
{
    #[inline(always)]
    fn from_f32(val: f32) -> Self {
        val.into()
    }
}

impl Real for f32 {}
impl Real for f64 {}
