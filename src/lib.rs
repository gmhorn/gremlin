//! # Gremlin
//!
//! Gremlin is a ray tracer

use std::ops::AddAssign;

use num_traits::{Float};

pub mod film;
pub mod geo;
pub mod spectrum;

/// Trait defining "real-valued" numbers.
/// 
/// Allows us to be generic over both `f32` and `f64` primitive types.
pub trait Real:
    Float + 
    AddAssign +
    From<f32>
{}

impl Real for f32 {}
impl Real for f64 {}