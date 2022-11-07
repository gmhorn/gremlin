use crate::Float;
use std::ops::{AddAssign, Div};

use super::Buffer;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FilmPixel<P> {
    total: P,
    count: u32,
}

impl<P> FilmPixel<P> {
    pub fn add_sample<S>(&mut self, sample: S)
    where
        P: From<S> + AddAssign,
    {
        self.total += P::from(sample);
        self.count += 1;
    }
}

impl<P> Buffer<FilmPixel<P>>
where
    P: Default + Copy + Div<Float, Output = P>,
{
    /// Creates a snapshot of this buffer's values.
    pub fn snapshot(&self) -> Buffer<P> {
        let mut buf = Buffer::<P>::new(self.width(), self.height());
        self.enumerate_pixels()
            .filter(|(_, _, &p)| p.count > 0)
            .for_each(|(x, y, &p)| {
                let avg = p.total / (p.count as Float);
                *buf.get_pixel_mut(x, y) = avg;
            });
        buf
    }
}
