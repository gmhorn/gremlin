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
//!
//! let mut img = RGBFilm::new(800, 600);
//! img.add_samples(|x, y| {
//!     RGB::from([x / 800.0, y / 600.0, 0.25])
//! });
//! img.to_snapshot().save_image("out.png").unwrap();
//! ```
//!
//! Raster space [`Buffer`]s runs from `(0, 0)` in the upper-left to
//! `(width-1, height-1)` in the lower right.

use crate::{
    color::{Color, LinearRGB, CIE1931, SRGB},
    geo::Coords,
    Float,
};
use image::{ImageResult, Rgb, RgbImage};
use rand::{Rng, rngs::ThreadRng, RngCore};
use rayon::prelude::*;
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

/// A typedef for raster-space coordinates.
///
/// Yes, just calling it _Raster_ and not something like _RasterPoint_ is
/// abusing the term "raster". The point of this typedef is to be convenient
/// shorthand for [`Coords<u32>`].
pub type Raster = Coords<u32>;

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

    /// The aspect ratio (`width`/`height`) of the buffer
    pub fn aspect_ratio(&self) -> Float {
        self.width as Float / self.height as Float
    }

    pub fn coordinates_of(&self, idx: usize) -> Raster {
        todo!();
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

    /// Create a function that can convert raster-space values to NDC-space
    /// values.
    pub fn raster_to_ndc(&self) -> impl Fn(Float, Float) -> (Float, Float) {
        let w = self.width as Float;
        let h = self.height as Float;
        move |x, y| (x / w, y / h)
    }

    /// Returns an iterator over the pixels.
    pub fn pixel_iter(&self) -> impl Iterator<Item = (Raster, &P)> {
        let width = self.width();
        self.iter().enumerate().map(move |(idx, pixel)| {
            let x = idx as u32 % width;
            let y = idx as u32 / width;
            (Raster::new(x, y), pixel)
        })
    }

    /// Returns an iterator over the pixels. Iterator allows mutating the pixel
    /// value.
    pub fn pixel_iter_mut(&mut self) -> impl Iterator<Item = (Raster, &mut P)> {
        let width = self.width();
        self.iter_mut().enumerate().map(move |(idx, pixel)| {
            let x = idx as u32 % width;
            let y = idx as u32 / width;
            (Raster::new(x, y), pixel)
        })
    }

    /// Returns a parallel iterator over the pixels.
    pub fn par_pixel_iter(&self) -> impl IndexedParallelIterator<Item = (Raster, &P)>
    where
        P: Sync,
    {
        let width = self.width();
        self.par_iter().enumerate().map(move |(idx, pixel)| {
            let x = idx as u32 % width;
            let y = idx as u32 / width;
            (Raster::new(x, y), pixel)
        })
    }

    /// Returns a parallel iterator over the pixels. Allows mutating the pixel
    /// value.
    pub fn par_pixel_iter_mut(&mut self) -> impl IndexedParallelIterator<Item = (Raster, &mut P)>
    where
        P: Send,
    {
        let width = self.width();
        self.par_iter_mut().enumerate().map(move |(idx, pixel)| {
            let x = idx as u32 % width;
            let y = idx as u32 / width;
            (Raster::new(x, y), pixel)
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

    /// Add a sample value to each pixel using the supplied function.
    ///
    /// The supplied function must be parallelizable, and is run across multiple
    /// pixels simultaneously. Often, the supplied function will be the main
    /// raytracing integrator, and effectively implementing a single pass of a
    /// main rendering loop.
    ///
    /// The values supplied to the function will be the raster-space values of
    /// the pixel (converted to [`Float`] for convenience). This makes it
    /// especially easy to pick a "random point in the pixel":
    ///
    /// ```no_run
    /// use gremlin::color::RGB;
    /// use gremlin::film::RGBFilm;
    /// use gremlin::Float;
    /// use rand::prelude::*;
    ///
    /// let mut img = RGBFilm::new(800, 600);
    /// img.add_samples(|x, y| {
    ///     let x = x + random::<Float>();
    ///     let y = y + random::<Float>();
    ///     pixel_color(x, y)
    /// });
    ///
    /// fn pixel_color(x: Float, y: Float) -> RGB {
    ///     todo!()
    /// }
    /// ```
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

    pub fn add_samples_2<F, S>(&mut self, func: F)
    where
        F: Fn(Raster, &mut ThreadRng) -> S + Sync,
        Color<CS>: From<S> + Send,
    {
        let width = self.width;
        self.par_iter_mut().enumerate().for_each_init(
            || rand::thread_rng(),
            |rng, (idx, pixel)| {
                let x = idx as u32 % width;
                let y = idx as u32 / width;
                pixel.add_sample(func(Raster::new(x, y), rng))
            },
        );
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
    fn add_samples() {}
}
