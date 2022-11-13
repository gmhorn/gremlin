use image::{ImageResult, Rgb, RgbImage};
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
    pub fn enumerate_raster(&self) -> impl Iterator<Item = (u32, u32, &P)> {
        self.pixels.iter().enumerate().map(|(idx, pixel)| {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;
            (x, y, pixel)
        })
    }

    /// Enumerates over the pixels of the image.
    ///
    /// Yields the raster-space coordinates of each pixel and a mutable
    /// reference to the pixel itself. Iteration order is left-to-right,
    /// top-to-bottom.
    #[inline]
    pub fn enumerate_raster_mut(&mut self) -> impl Iterator<Item = (u32, u32, &mut P)> {
        self.pixels.iter_mut().enumerate().map(|(idx, pixel)| {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;
            (x, y, pixel)
        })
    }

    /// Enumerates over the pixels of the image.
    ///
    /// Yields a random NDC-space coordinate within each pixel and a reference
    /// to the pixel itself. Iteration order is left-to-right, top-to-bottom.
    #[inline]
    pub fn enumerate_ndc(&self) -> impl Iterator<Item = (Float, Float, &P)> {
        let width = self.width as Float;
        let height = self.height as Float;

        self.pixels.iter().enumerate().map(move |(idx, pixel)| {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;

            let u = ((x as Float) + rand::random::<Float>()) / width;
            let v = ((y as Float) + rand::random::<Float>()) / height;
            (u, v, pixel)
        })
    }

    /// Enumerates over the pixels of the image.
    ///
    /// Yields a random NDC-space coordinate within each pixel and a mutable
    /// reference to the pixel itself. Iteration order is left-to-right,
    /// top-to-bottom.
    #[inline]
    pub fn enumerate_ndc_mut(&mut self) -> impl Iterator<Item = (Float, Float, &mut P)> {
        self.pixels.iter_mut().enumerate().map(|(idx, pixel)| {
            let x = (idx as u32) % self.width;
            let y = (idx as u32) / self.width;

            let width = self.width as Float;
            let height = self.height as Float;

            let u = ((x as Float) + rand::random::<Float>()) / width;
            let v = ((y as Float) + rand::random::<Float>()) / height;
            (u, v, pixel)
        })
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
    /// The image format is derived from the file extension. Does not perform
    /// any gamma correction.
    fn save_image<Q>(&self, path: Q) -> ImageResult<()>
    where
        Q: AsRef<std::path::Path>,
    {
        RgbImage::from_fn(self.width, self.height, |x, y| {
            Rgb::<u8>::from(*self.get_pixel(x, y))
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
        let mut iter = buf.enumerate_raster();

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
