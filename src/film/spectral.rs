use crate::spectrum::Sampled;

use super::{XYZ, Buffer, ColorMatchingCurves};

/// A pixel that aggregates spectral radiosity values.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SpectralPixel {
    total: XYZ,
    count: u32,
}

impl SpectralPixel {
    pub fn add_sample(&mut self, sample: Sampled) {

    }
}

// BUFFER IMPLEMENTATIONS

/// A buffer whose pixels can aggregate spectral radiosity values.
pub type SpectralBuffer = Buffer<SpectralPixel>;

impl Buffer<SpectralPixel> {

}

#[cfg(test)]
mod tests {
    use crate::spectrum;
    use super::*;

    #[test]
    fn interface() {
        let spec = Sampled::from(|w| spectrum::blackbody(6500.0, w));
        let mut buf = SpectralBuffer::new(10, 5);
        
        buf.get_pixel_mut(0, 1).add_sample(spec);
    }
}