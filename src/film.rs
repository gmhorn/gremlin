//! # Buffers, pixels, and films.
//!
//! This module contains the basic building blocks needed for collecting and
//! aggregating radiosity samples, and converting to final images.
//!
//! [`Buffer`] is the base struct used throughout this package. It is heap-
//! allocated rectangular grid of (generic) pixels. Most of the time, it's
//! easier to use the [`RGBFilm`] or [`SpectralFilm`] typedefs, as they support
//! the most common operations needed for raytracing (aggregating color values
//! on a per-pixel basis, and taking snapshots of average pixel values ).
//!
//! ```no_run
//! use gremlin::film::RGBFilm;
//! use gremlin::color::RGB;
//! use gremlin::Float;
//!
//! let mut img = RGBFilm::new(800, 600);
//! img.pixel_iter_mut().for_each(|(px, py, pixel)| {
//!     let color = RGB::from([px as Float / 800.0, py as Float / 600.0, 0.25]);
//!     pixel.add_sample(color);
//! });
//! img.to_snapshot().save_image("out.png").unwrap();
//! ```
//!
//! Raster space for the various pixel iteration methods runs from `(0, 0)` in
//! the upper-left to `(width-1, height-1)` in the lower right.

use crate::{
    color::{Color, LinearRGB, CIE1931, SRGB},
    Float,
};
use image::{ImageResult, Rgb, RgbImage};
use rayon::prelude::*;
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

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
        P: Default + Clone,
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

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
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
        P: SRGB,
    {
        RgbImage::from_fn(self.width, self.height, |x, y| {
            let idx = ((y * self.width) + x) as usize;
            Rgb::<u8>::from(self.pixels[idx].to_srgb())
        })
        .save(path)
    }

    /// Returns an iterator over the pixels.
    pub fn pixel_iter(&self) -> impl Iterator<Item = (u32, u32, &P)> {
        let width = self.width();
        self.iter().enumerate().map(move |(idx, pixel)| {
            let px = idx as u32 % width;
            let py = idx as u32 / width;
            (px, py, pixel)
        })
    }

    /// Returns an iterator over the pixels. Iterator allows mutating the pixel
    /// value.
    pub fn pixel_iter_mut(&mut self) -> impl Iterator<Item = (u32, u32, &mut P)> {
        let width = self.width();
        self.iter_mut().enumerate().map(move |(idx, pixel)| {
            let px = idx as u32 % width;
            let py = idx as u32 / width;
            (px, py, pixel)
        })
    }

    /// Returns a parallel iterator over the pixels.
    pub fn par_pixel_iter(&self) -> impl IndexedParallelIterator<Item = (u32, u32, &P)>
    where
        P: Sync,
    {
        let width = self.width();
        self.par_iter().enumerate().map(move |(idx, pixel)| {
            let px = idx as u32 % width;
            let py = idx as u32 / width;
            (px, py, pixel)
        })
    }

    /// Returns a parallel iterator over the pixels. Allows mutating the pixel
    /// value.
    pub fn par_pixel_iter_mut(&mut self) -> impl IndexedParallelIterator<Item = (u32, u32, &mut P)>
    where
        P: Send,
    {
        let width = self.width();
        self.par_iter_mut().enumerate().map(move |(idx, pixel)| {
            let px = idx as u32 % width;
            let py = idx as u32 / width;
            (px, py, pixel)
        })
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
        Color<CS>: From<S>,
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
}
