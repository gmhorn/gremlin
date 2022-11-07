use super::{Buffer, Save, XYZBuffer, XYZ};
use crate::{spectrum::Sampled, Float};

/// A pixel that aggregates spectral radiosity values.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SpectralPixel {
    total: XYZ,
    count: u32,
}

impl SpectralPixel {
    #[inline]
    pub fn add_sample(&mut self, sample: &Sampled) {
        self.total += XYZ::from(sample);
        self.count += 1;
    }
}

// BUFFER IMPLEMENTATIONS

/// A buffer whose pixels can aggregate spectral radiosity values.
pub type SpectralBuffer = Buffer<SpectralPixel>;

impl Buffer<SpectralPixel> {
    /// Creates a snapshot of this buffer's values.
    pub fn snapshot(&self) -> XYZBuffer {
        let mut buf = XYZBuffer::new(self.width(), self.height());
        self.enumerate_pixels()
            .filter(|(_, _, &p)| p.count > 0)
            .for_each(|(x, y, &p)| {
                let val = p.total / (p.count as Float);
                *buf.get_pixel_mut(x, y) = val;
            });
        buf
    }
}

impl Save for Buffer<SpectralPixel> {
    #[inline]
    fn save_image<P>(&self, path: P) -> image::ImageResult<()>
    where
        P: AsRef<std::path::Path>,
    {
        self.snapshot().save_image(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrum;

    #[test]
    fn interface() {
        let spec = Sampled::from(|w| spectrum::blackbody(6500.0, w));
        let mut buf = SpectralBuffer::new(10, 5);

        buf.get_pixel_mut(0, 1).add_sample(&spec);
    }
}
