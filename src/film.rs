//! # Film module
//!
//! Implements functionality for aggregating radiosity samples collected during
//! ray tracing, and converting to final images.

use image::ImageResult;
use std::path::Path;

mod buffer;
pub use buffer::*;

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
