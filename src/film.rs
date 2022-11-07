//! # Film module
//!
//! Implements functionality for aggregating radiosity samples collected during
//! ray tracing, and converting to final images.

mod buffer;
use std::path::Path;

pub use buffer::*;

mod consts;

mod pixel_old;
use image::ImageResult;
pub use pixel_old::*;

mod spectral;
pub use spectral::*;

mod xyz;
pub use xyz::*;

/// Used for saving an image to disk.
pub trait Save {
    /// Saves an image to disk, at the path specified.
    fn save_image<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>;
}