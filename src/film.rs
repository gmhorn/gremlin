//! # Film module
//!
//! Implements functionality for aggregating radiosity samples collected during
//! ray tracing, and converting to final images.
//! 
//! [`Buffer`] is the base struct used throughout this package. It is a generic
//! rectangular container for pixels.

use image::ImageResult;
use std::path::Path;

mod buffer;
pub use buffer::*;

mod pixel;
pub use pixel::*;

mod rgb;
pub use rgb::*;

mod xyz;
pub use xyz::*;

// TYPE DEFINITIONS

/// A buffer whose pixels are RGB values.
pub type RGBBuffer = Buffer<RGB>;

/// A film that takes RGB samples.
pub type RGBFilm = Buffer<FilmPixel<RGB>>;

/// A buffer whose pixels are XYZ values.
pub type XYZBuffer = Buffer<XYZ>;

/// A film that takes spectral samples.
pub type SpectralFilm = Buffer<FilmPixel<XYZ>>;

/// Used for saving an image to disk.
pub trait Save {
    /// Saves an image to disk, at the path specified.
    fn save_image<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>;
}
