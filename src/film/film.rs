use std::ops::{Deref, DerefMut};

use super::Pixel;

/// Film is a rectangular grid of [`Pixel`]s.
///
/// It supports various
pub struct Film {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Film {
    /// Create a new Film with the given width and height
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
            pixels: vec![Pixel::new(); (width * height) as usize],
        }
    }

    /// The width and height of the film.
    #[inline]
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// The width of the film.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the film.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// The aspect ratio (width / height) of the film.
    #[inline]
    pub fn aspect_ratio(&self) -> f64 {
        (self.width as f64) / (self.height as f64)
    }

    /// Enumerates over the pixels of the image.
    ///
    /// Yields the raster-space coordinates of each pixel and a reference to the
    /// pixel itself. Iteration order is left-to-right, top-to-bottom.
    #[inline]
    pub fn enumerate_pixels(&self) -> EnumeratePixels {
        EnumeratePixels {
            pixels: self.pixels.iter(),
            width: self.width,
            current: 0,
        }
    }

    /// Enumerates over the pixels of the image.
    ///
    /// Yields the raster-space coordinates of each pixel and a mutable
    /// reference to the pixel itself. Iteration order is left-to-right,
    /// top-to-bottom.
    #[inline]
    pub fn enumerate_pixels_mut(&mut self) -> EnumeratePixelsMut {
        EnumeratePixelsMut {
            pixels: self.pixels.iter_mut(),
            width: self.width,
            current: 0,
        }
    }
}

// DEREFS

impl Deref for Film {
    type Target = [Pixel];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl DerefMut for Film {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

// ITERATORS

impl IntoIterator for Film {
    type Item = Pixel;
    type IntoIter = std::vec::IntoIter<Pixel>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

// ENUMERATIONS

/// Enumerate pixels of the film.
pub struct EnumeratePixels<'a> {
    pixels: std::slice::Iter<'a, Pixel>,
    width: u32,
    current: usize,
}

impl<'a> Iterator for EnumeratePixels<'a> {
    type Item = (u32, u32, &'a Pixel);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let pixel = self.pixels.next()?;
        let idx = self.current as u32;

        self.current += 1;
        Some((idx % self.width, idx / self.width, pixel))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pixels.size_hint()
    }
}

impl<'a> ExactSizeIterator for EnumeratePixels<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.pixels.len()
    }
}

/// Enumerates mutable pixels of the film.
pub struct EnumeratePixelsMut<'a> {
    pixels: std::slice::IterMut<'a, Pixel>,
    width: u32,
    current: usize,
}

impl<'a> Iterator for EnumeratePixelsMut<'a> {
    type Item = (u32, u32, &'a mut Pixel);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let pixel = self.pixels.next()?;
        let idx = self.current as u32;

        self.current += 1;
        Some((idx % self.width, idx / self.width, pixel))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.pixels.size_hint()
    }
}

impl<'a> ExactSizeIterator for EnumeratePixelsMut<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.pixels.len()
    }
}
