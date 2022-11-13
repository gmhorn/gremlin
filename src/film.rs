//! # Film module
//!
//! Implements functionality for aggregating radiosity samples collected during
//! ray tracing, and converting to final images.
//!
//! [`Buffer`] is the base struct used throughout this package. It is a generic
//! rectangular container for pixels. By bringing the [`Save`] trait into scope,
//! buffers may be saved to disk (as long as the underlying pixel type supports
//! conversion to packed [`u8`] RGB format).
//!
//! Two color representations are supported: [`RGB`] and [`XYZ`]. Both are
//! tristimulus and [`Float`][crate::Float]-valued, with a gamut of `[0, 1]`.
//! [`XYZ`] are treated as CIE 1931 colorspace values. [`RGB`] are not treated
//! as being part of any particular color space.
//!
//! The [`FilmPixel`] struct supports computing a final color by averaging
//! multiple samples. To make things convenient, this package exports a number
//! of typedefs. The general pattern is a `Buffer<Foo>` is exported as
//! `FooBuffer`, while a `Buffer<FilmPixel<Foo>>` is exported as `FooFilm`.

use image::ImageResult;
use std::path::Path;

mod buffer;
pub use buffer::*;

mod iter;
pub use iter::*;

mod pixel;
pub use pixel::*;

mod rgb;
pub use rgb::*;

mod xyz;
pub use xyz::*;

// TYPE DEFINITIONS

/// A film that's generic over pixel type.
pub type Film<P> = Buffer<FilmPixel<P>>;

/// A buffer whose pixels are RGB values.
pub type RGBBuffer = Buffer<RGB>;

/// A film that takes RGB samples.
pub type RGBFilm = Film<RGB>;

/// A buffer whose pixels are XYZ values.
pub type XYZBuffer = Buffer<XYZ>;

/// A film that takes spectral samples.
pub type SpectralFilm = Film<XYZ>;

/// Used for saving an image to disk.
pub trait Save {
    /// Saves an image to disk, at the path specified.
    fn save_image<P>(&self, path: P) -> ImageResult<()>
    where
        P: AsRef<Path>;
}
