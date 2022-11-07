use std::ops::{Deref, DerefMut};

/// A rectangular grid of pixels.
///
/// It supports various operations for iteration and pixel retrieval.
/// Specializations may support additional operations, such as conversion
/// between color spaces and saving to disk.
/// 
/// Raster space extends from `(0, 0)` at the top-left to `(width-1, height-1)`
/// at the bottom right.
#[derive(Debug)]
pub struct Buffer<P> {
    width: u32,
    height: u32,
    pixels: Vec<P>,
}

// Constructors

impl<P: Default + Clone> Buffer<P> {
    /// Create a new Film with the given width and height.
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![P::default(); (width * height) as usize];
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl<P> Buffer<P> {
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

    /// Get a pixel at the given `(x, y)` coordinates.
    /// 
    /// # Panics
    /// 
    /// If `x >= width` or `y >= height`.
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> &P {
        let idx = (y * self.height) + x;
        &self.pixels[idx as usize]
    }

    /// Get a pixel at the given `(x, y)` coordinates.
    #[inline]
    pub fn get_pixel_checked(&self, x: u32, y: u32) -> Option<&P> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.get_pixel(x, y))
        }
    }

    /// Enumerates over the pixels of the image.
    ///
    /// Yields the raster-space coordinates of each pixel and a reference to the
    /// pixel itself. Iteration order is left-to-right, top-to-bottom.
    #[inline]
    pub fn enumerate_pixels(&self) -> EnumeratePixels<P> {
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
    pub fn enumerate_pixels_mut(&mut self) -> EnumeratePixelsMut<P> {
        EnumeratePixelsMut {
            pixels: self.pixels.iter_mut(),
            width: self.width,
            current: 0,
        }
    }
}

// DEREFS

impl<P> Deref for Buffer<P> {
    type Target = [P];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

impl<P> DerefMut for Buffer<P> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pixels
    }
}

// ITERATORS

impl<P> IntoIterator for Buffer<P> {
    type Item = P;
    type IntoIter = std::vec::IntoIter<P>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

// ENUMERATIONS

/// Enumerate pixels of the film.
pub struct EnumeratePixels<'a, P> {
    pixels: std::slice::Iter<'a, P>,
    width: u32,
    current: usize,
}

impl<'a, P> Iterator for EnumeratePixels<'a, P> {
    type Item = (u32, u32, &'a P);

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

impl<'a, P> ExactSizeIterator for EnumeratePixels<'a, P> {
    #[inline]
    fn len(&self) -> usize {
        self.pixels.len()
    }
}

/// Enumerates mutable pixels of the film.
pub struct EnumeratePixelsMut<'a, P> {
    pixels: std::slice::IterMut<'a, P>,
    width: u32,
    current: usize,
}

impl<'a, P> Iterator for EnumeratePixelsMut<'a, P> {
    type Item = (u32, u32, &'a mut P);

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

impl<'a, P> ExactSizeIterator for EnumeratePixelsMut<'a, P> {
    #[inline]
    fn len(&self) -> usize {
        self.pixels.len()
    }
}
