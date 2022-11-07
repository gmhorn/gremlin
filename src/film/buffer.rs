use image::{Rgb, RgbImage};

use crate::Float;

use super::Save;

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

impl<P: Default + Copy> Buffer<P> {
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
    pub fn aspect_ratio(&self) -> Float {
        (self.width as Float) / (self.height as Float)
    }

    /// Get a pixel at the given `(x, y)` coordinates.
    ///
    /// # Panics
    ///
    /// If `x >= width` or `y >= height`.
    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> &P {
        let idx = self.index_of(x, y);
        &self.pixels[idx]
    }

    /// Get a pixel at the given `(x, y)` coordinates.
    #[inline]
    pub fn get_pixel_checked(&self, x: u32, y: u32) -> Option<&P> {
        let idx = self.index_of_checked(x, y)?;
        Some(&self.pixels[idx])
    }

    /// Get a pixel at the given `(x, y)` coordinates.
    ///
    /// # Panics
    ///
    /// If `x >= width` or `y >= height`.
    #[inline]
    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> &mut P {
        let idx = self.index_of(x, y);
        &mut self.pixels[idx]
    }

    /// Get a pixel at the given `(x, y)` coordinates.
    #[inline]
    pub fn get_pixel_mut_checked(&mut self, x: u32, y: u32) -> Option<&mut P> {
        let idx = self.index_of_checked(x, y)?;
        Some(&mut self.pixels[idx])
    }

    #[inline(always)]
    fn index_of(&self, x: u32, y: u32) -> usize {
        ((y * self.width) + x) as usize
    }

    #[inline(always)]
    fn index_of_checked(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.index_of(x, y))
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

// TRAIT IMPLS

impl<P> Save for Buffer<P>
where
    Rgb<u8>: From<P>,
    P: Copy,
{
    /// Saves the buffer to a file at the path specified.
    ///
    /// The image format is derived from the file extension. Assumes a sRGB
    /// color space.
    fn save_image<Q>(&self, path: Q) -> image::ImageResult<()>
    where
        Q: AsRef<std::path::Path>,
    {
        RgbImage::from_fn(self.width, self.height, |x, y| {
            Rgb::<u8>::from(*self.get_pixel(x, y)).into()
        })
        .save(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_pixels() {
        let buf: Buffer<i32> = Buffer::new(3, 2);
        let mut iter = buf.enumerate_pixels();

        assert_eq!(Some((0_u32, 0_u32, &0_i32)), iter.next());
        assert_eq!(Some((1_u32, 0_u32, &0_i32)), iter.next());
        assert_eq!(Some((2_u32, 0_u32, &0_i32)), iter.next());
        assert_eq!(Some((0_u32, 1_u32, &0_i32)), iter.next());
        assert_eq!(Some((1_u32, 1_u32, &0_i32)), iter.next());
        assert_eq!(Some((2_u32, 1_u32, &0_i32)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn get_pixel() {
        let mut buf: Buffer<i64> = Buffer::new(10, 5);
        assert_eq!(0, *buf.get_pixel(0, 1));

        *buf.get_pixel_mut(0, 1) = 10;
        assert_eq!(10, *buf.get_pixel(0, 1));
    }
}
