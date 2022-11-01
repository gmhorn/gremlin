use crate::Real;

// CONSTANTS

mod consts {
    pub const MIN: f32 = 380.0;
    pub const MAX: f32 = 780.0;
    pub const STEP: f32 = 5.0;
    pub const COUNT: usize = ((MAX - MIN) / STEP) as usize;
}

// TYPE DEFINITIONS

/// A 32-bit sampled spectrum.
pub type Sampled32 = Sampled<f32>;

/// A 64-bit sampled spectrum.
pub type Sampled64 = Sampled<f64>;

// STRUCT DEFINITION

/// A spectrum with values defined at discrete points.
pub struct Sampled<R>([R; consts::COUNT]);

impl<R: Real> Sampled<R> {
    /// Creates a new sampled spectrum with all values zero.
    #[inline]
    pub fn zero() -> Self {
        Self::splat(R::zero())
    }

    /// Creates a new sampled spectrum with all values equal.
    #[inline]
    pub const fn splat(value: R) -> Self {
        Self([value; consts::COUNT])
    }

    /// Creates a new sampled spectrum by repeated application of the given
    /// function.
    ///
    /// The argument to the function is the wavelength.
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(R) -> R,
    {
        let mut spec = Self::zero();
        for (wavelength, val) in spec.enumerate_values_mut() {
            *val = f(wavelength)
        }
        spec
    }

    /// Enumerates over the sampled spectrum.
    ///
    /// Yields pairs `(wavelength, &value)`.
    #[inline]
    pub fn enumerate_values(&self) -> EnumerateValues<R> {
        EnumerateValues {
            values: self.0.iter(),
            current: consts::MIN.into(),
            step: consts::STEP.into(),
        }
    }

    /// Enumerates over the sampled spectrum.
    ///
    /// Yields pairs `(wavelength, &mut value)`.
    #[inline]
    pub fn enumerate_values_mut(&mut self) -> EnumerateValuesMut<R> {
        EnumerateValuesMut {
            values: self.0.iter_mut(),
            current: consts::MIN.into(),
            step: consts::STEP.into(),
        }
    }
}

// ENUMERATIONS

/// Enumerates `(wavelength, value)` pairs.
pub struct EnumerateValues<'a, R> {
    values: std::slice::Iter<'a, R>,
    current: R,
    step: R,
}

impl<'a, R: Real> Iterator for EnumerateValues<'a, R> {
    type Item = (R, &'a R);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let y = self.values.next()?;
        let x = self.current;

        self.current += self.step;
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }
}

impl<'a, R: Real> ExactSizeIterator for EnumerateValues<'a, R> {
    #[inline]
    fn len(&self) -> usize {
        self.values.len()
    }
}

/// Enumerates mutable `(wavelength, value)` pairs.
pub struct EnumerateValuesMut<'a, R> {
    values: std::slice::IterMut<'a, R>,
    current: R,
    step: R,
}

impl<'a, R: Real> Iterator for EnumerateValuesMut<'a, R> {
    type Item = (R, &'a mut R);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let y = self.values.next()?;
        let x = self.current;

        self.current += self.step;
        Some((x, y))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.values.size_hint()
    }
}

impl<'a, R: Real> ExactSizeIterator for EnumerateValuesMut<'a, R> {
    #[inline]
    fn len(&self) -> usize {
        self.values.len()
    }
}

// CONVERSIONS: OTHER -> SPECTRUM

impl<R: Real> From<[R; consts::COUNT]> for Sampled<R> {
    #[inline]
    fn from(arr: [R; consts::COUNT]) -> Self {
        Self(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_values() {
        let s = Sampled64::zero();
        let mut e = s.enumerate_values();

        let (wavelength, &value) = e.next().unwrap();
        assert_eq!(380.0, wavelength);
        assert_eq!(0.0, value);

        let (wavelength, &value) = e.next().unwrap();
        assert_eq!(385.0, wavelength);
        assert_eq!(0.0, value);
    }
}
