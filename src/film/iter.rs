use crate::Float;

use super::EnumeratePixels;

pub struct AsNDC<'a, I> {
    iter: &'a mut I,
    width: Float,
    height: Float,
}

impl<'a, P> Iterator for AsNDC<'a, EnumeratePixels<'a, P>> {
    type Item = (Float, Float, &'a P);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y, pixel) = self.iter.next()?;

        let u = ((x as Float) + 0.5) / self.width;
        let v = ((y as Float) + 0.5) / self.height;
        Some((u, v, pixel))
    }
}