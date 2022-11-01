//! # Film module
//!
//! Implements functionality for aggregating radiosity samples collected during
//! ray tracing, and converting to final images.

mod film;
pub use film::*;

mod pixel;
pub use pixel::*;
