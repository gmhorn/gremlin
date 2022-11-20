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

use image::{ImageResult, RgbImage, Rgb};
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};
use rayon::prelude::*;
use crate::{Float, color::{Color, LinearRGB, CIE1931}};

/// A rectangular grid of pixels.
pub struct Buffer<P> {
    width: u32,
    height: u32,
    pixels: Vec<P>,
}

impl<P> Buffer<P> {
    /// Create a new buffer with the given width and height.
    ///
    /// All pixels are initialized with their default value.
    pub fn new(width: u32, height: u32) -> Self
    where
        P: Default + Clone
    {
        let pixels = vec![P::default(); (width * height) as usize];
        Self {
            width,
            height,
            pixels,
        }
    }

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

    /// Save the buffer as an image at the path specified.
    /// 
    /// Image format is derived from the file extension.
    pub fn save_image<Q>(&self, path: Q) -> ImageResult<()>
    where
        Q: AsRef<Path>,
        P: Into<[u8; 3]> + Copy
    {
        RgbImage::from_fn(self.width, self.height, |x, y| {
            let idx = ((y * self.width) + x) as usize;
            Rgb::<u8>::from(self.pixels[idx].into())
        })
        .save(path)
    }
}

// DEREFS

impl<P> Deref for Buffer<P> {
    type Target = [P];

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl<P> DerefMut for Buffer<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

/// A pixel that aggregates values from a given color space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pixel<CS> {
    sum: Color<CS>,
    count: u32,
}

impl<CS: Copy> Pixel<CS> {
    /// Get the color value representing the average over all samples.
    #[inline]
    pub fn to_color(&self) -> Color<CS> {
        self.sum / (self.count as Float).max(1.0)
    }

    /// Add a sample to this pixel.
    #[inline]
    pub fn add_sample<S>(&mut self, sample: S)
    where
        Color<CS>: From<S>
    {
        self.sum += sample.into();
        self.count += 1;
    }
}

/// Convenience typedef for a buffer of pixels in a given color space.
pub type Film<CS> = Buffer<Pixel<CS>>;

/// A film with [`RGB`] pixels.
/// 
/// [`RGB`]: ::crate::color::RGB
pub type RGBFilm = Buffer<Pixel<LinearRGB>>;

/// A film with [`XYZ`] pixels.
/// 
/// [`XYZ`]: ::crate::color::XYZ
pub type SpectralFilm = Buffer<Pixel<CIE1931>>;

impl<CS: Copy> Buffer<Pixel<CS>> {
    /// Creates a snapshot of the buffer's values.
    pub fn to_snapshot(&self) -> Buffer<Color<CS>> {
        Buffer {
            width: self.width,
            height: self.height,
            pixels: self.pixels.iter().map(|p| p.to_color()).collect(),
        }
    }

    pub fn add_samples<F, S>(&mut self, func: F)
    where
        F: Fn(Float, Float) -> S + Sync,
        Color<CS>: From<S> + Send,
    {
        let width = self.width;
        self.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
            let x = idx as u32 % width;
            let y = idx as u32 / width;
            pixel.add_sample(func(x as Float, y as Float))
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::{RGB, XYZ};

    #[test]
    fn pixel_aggregation() {
        let mut pix = Pixel::default();
        pix.add_sample(RGB::from([1.0, 1.0, 1.0]));
        assert_eq!(pix.to_color(), RGB::from([1.0, 1.0, 1.0]));

        pix.add_sample(RGB::from([0.0, 0.0, 0.0]));
        assert_eq!(pix.to_color(), RGB::from([0.5, 0.5, 0.5]));
    }

    #[test]
    fn add_sample_conv() {
        let mut pix = Pixel::default();

        struct Uniform(Float);
        impl From<Uniform> for XYZ {
            fn from(val: Uniform) -> Self {
                XYZ::from([val.0, val.0, val.0])
            }
        }

        pix.add_sample(Uniform(1.0));
        pix.add_sample(Uniform(0.0));
        assert_eq!(XYZ::from([0.5, 0.5, 0.5]), pix.to_color());
    }

    #[test]
    fn add_samples() {
    }
}