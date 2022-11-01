//! # Film module
//!
//! Implements functionality for aggregating radiosity samples collected during
//! ray tracing, and converting to final images.

mod buffer;
pub use buffer::*;

mod pixel;
pub use pixel::*;
