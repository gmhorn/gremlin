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
use std::{
    ops::{AddAssign, Deref, DerefMut, Div},
    path::Path,
};

mod buffer;
pub use buffer::*;

mod pixel;
pub use pixel::*;

mod rgb;
pub use rgb::*;

mod xyz;
pub use xyz::*;

use crate::Float;

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

/// A color value.
pub trait Color: Default + Copy + AddAssign + Div<Float, Output = Self> {}

impl<C: Default + Copy + AddAssign + Div<Float, Output = Self>> Color for C {}

/// A pixel that aggregates color values.
pub struct Pixel<C> {
    sum: C,
    count: u32,
}

impl<C: Color> Pixel<C> {
    pub fn to_color(&self) -> C {
        self.sum / (self.count as Float)
    }
}

/// A rectangular grid of pixels.
pub struct Buf<P> {
    width: u32,
    height: u32,
    pixels: Vec<P>,
}

impl<P> Buf<P> {
    /// The width of the buffer
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the buffer
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The aspect ratio (`width`/`height`) of the buffer
    pub fn aspect_ratio(&self) -> Float {
        self.width as Float / self.height as Float
    }
}

impl<P: Default + Clone> Buf<P> {
    /// Create a new buffer with the given width and height.
    ///
    /// All pixels are initialized with their default value.
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![P::default(); (width * height) as usize];
        Self {
            width,
            height,
            pixels,
        }
    }
}

// DEREFS

impl<P> Deref for Buf<P> {
    type Target = [P];

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl<P> DerefMut for Buf<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

impl<C: Color> Buf<Pixel<C>> {
    pub fn to_snapshot(&self) -> Buf<C> {
        Buf {
            width: self.width,
            height: self.height,
            pixels: self.pixels.iter().map(|p| p.to_color()).collect(),
        }
    }
}
