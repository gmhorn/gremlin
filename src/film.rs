//! # Film module
//!
//! Implements functionality for aggregating radiosity samples collected during
//! ray tracing, and converting to final images.

mod buffer;
pub use buffer::*;

mod consts;

mod pixel_old;
pub use pixel_old::*;

mod xyz;
pub use xyz::*;
