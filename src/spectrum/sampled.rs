use crate::{Float, math::PiecewiseLinearFn};
use std::ops::{Deref, DerefMut};

// CONSTANTS
mod consts {
    use crate::Float;

    pub const MIN: Float = 380.0;
    pub const MAX: Float = 780.0;
    pub const STEP: Float = 5.0;
    pub const COUNT: usize = ((MAX - MIN) / STEP) as usize;
}

/// A spectrum with values defined at discrete points.
///
/// The sample wavelengths are uniform, compile-time constants. This
/// significantly reduces the implementation complexity and improves
/// performance, since linear operations such as addition and riemann summation
/// can be implemented with straightforward iteration.
///
/// Each value represents (conceptually, is the "average value") of the spectrum
/// in a range of wavelengths. For example, if the minimum wavelength is `380nm`
/// and the step size is `5nm`, then the first value represents the wavelength
/// range `[380, 385)`, the second `[385, 390)`, etc.
///
/// Possible future improvements would be to make the minimum, maximum, and
/// step size constants configurable via Cargo.
///
/// See: <https://pbr-book.org/3ed-2018/Color_and_Radiometry/The_SampledSpectrum_Class>
pub struct Sampled([Float; consts::COUNT]);

impl Sampled {
    /// Creates a new sampled spectrum with the given values.
    #[inline]
    pub const fn new(values: [Float; consts::COUNT]) -> Self {
        Self(values)
    }

    /// Creates a new sampled spectrum with all values zero.
    #[inline]
    pub const fn zero() -> Self {
        Self::splat(0.0)
    }

    /// Creates a new sampled spectrum with all values equal.
    #[inline]
    pub const fn splat(value: Float) -> Self {
        Self([value; consts::COUNT])
    }

    /// Creates a new sampled spectrum by repeated application of the given
    /// function.
    ///
    /// The arguments to the function is the half-open wavelength interval
    /// `[w0, w1)`.
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(Float, Float) -> Float,
    {
        let mut spec = Self::zero();
        for (wavelength, val) in spec.enumerate_values_mut() {
            *val = f(wavelength, wavelength + consts::STEP)
        }
        spec
    }

    /// Enumerates over the sampled spectrum.
    ///
    /// Yields pairs `(wavelength, &value)`.
    #[inline]
    pub fn enumerate_values(&self) -> EnumerateValues {
        EnumerateValues {
            values: self.0.iter(),
            current: consts::MIN,
        }
    }

    /// Enumerates over the sampled spectrum.
    ///
    /// Yields pairs `(wavelength, &mut value)`.
    #[inline]
    pub fn enumerate_values_mut(&mut self) -> EnumerateValuesMut {
        EnumerateValuesMut {
            values: self.0.iter_mut(),
            current: consts::MIN,
        }
    }
}

// DEREFS

impl Deref for Sampled {
    type Target = [Float];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sampled {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// ENUMERATIONS

/// Enumerates `(wavelength, value)` pairs.
pub struct EnumerateValues<'a> {
    values: std::slice::Iter<'a, Float>,
    current: Float,
}

impl<'a> Iterator for EnumerateValues<'a> {
    type Item = (Float, &'a Float);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let y = self.values.next()?;
        let x = self.current;

        self.current += consts::STEP;
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }
}

impl<'a> ExactSizeIterator for EnumerateValues<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.values.len()
    }
}

/// Enumerates mutable `(wavelength, value)` pairs.
pub struct EnumerateValuesMut<'a> {
    values: std::slice::IterMut<'a, Float>,
    current: Float,
}

impl<'a> Iterator for EnumerateValuesMut<'a> {
    type Item = (Float, &'a mut Float);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let y = self.values.next()?;
        let x = self.current;

        self.current += consts::STEP;
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }
}

impl<'a> ExactSizeIterator for EnumerateValuesMut<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.values.len()
    }
}

// CONVERSIONS: OTHER -> SPECTRUM

impl From<[Float; consts::COUNT]> for Sampled {
    #[inline]
    fn from(values: [Float; consts::COUNT]) -> Self {
        Self::new(values)
    }
}

impl<F> From<F> for Sampled
where
    F: Fn(Float) -> Float,
{
    /// Creates a sampled spectrum by evaluating a function.
    ///
    /// There's a subtlety here. Each sampled wavelength represents a half-open
    /// range of wavelengths `[w0, w1)`. This evaluates the value at `w0`. That
    /// is, it samples from the beginning of the range. If a different sampling
    /// method is required (*e.g.* taking the average over the range) then
    /// [`from_fn`][Self::from_fn] should be used, which provides the closure
    /// with both ends of the wavelength range.
    /// 
    /// # Example
    /// 
    /// ```
    /// use gremlin::spectrum::Sampled;
    /// 
    /// let _ = Sampled::from(|w| w + 1.0);
    /// ```
    #[inline]
    fn from(f: F) -> Self {
        let mut spec = Self::zero();
        for (wavelength, val) in spec.enumerate_values_mut() {
            *val = f(wavelength)
        }
        spec
    }
}

impl From<PiecewiseLinearFn> for Sampled {
    /// Converts
    #[inline]
    fn from(f: PiecewiseLinearFn) -> Self {
        Self::from_fn(|w0, w1| f.integrate(w0, w1) / consts::STEP)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_values() {
        let s = Sampled::zero();
        let mut e = s.enumerate_values();

        let (wavelength, &value) = e.next().unwrap();
        assert_eq!(380.0, wavelength);
        assert_eq!(0.0, value);

        let (wavelength, &value) = e.next().unwrap();
        assert_eq!(385.0, wavelength);
        assert_eq!(0.0, value);
    }
}
