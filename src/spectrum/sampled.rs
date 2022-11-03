use std::ops::{Deref, DerefMut};
use crate::MyFloat;

use super::Continuous;

// CONSTANTS
mod consts {
    use crate::MyFloat;

    pub const MIN: MyFloat = 380.0;
    pub const MAX: MyFloat = 780.0;
    pub const STEP: MyFloat = 5.0;
    pub const COUNT: usize = ((MAX - MIN) / STEP) as usize;
}

/// A spectrum with values defined at discrete points.
pub struct Sampled([MyFloat; consts::COUNT]);

impl Sampled {
    /// Creates a new sampled spectrum with all values zero.
    #[inline]
    pub const fn zero() -> Self {
        Self::splat(0.0)
    }

    /// Creates a new sampled spectrum with all values equal.
    #[inline]
    pub const fn splat(value: MyFloat) -> Self {
        Self([value; consts::COUNT])
    }

    /// Creates a new sampled spectrum by repeated application of the given
    /// function.
    ///
    /// The argument to the function is the wavelength.
    pub fn from_fn<F>(mut f: F) -> Self
    where
        F: FnMut(MyFloat) -> MyFloat,
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
    type Target = [MyFloat];

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
    values: std::slice::Iter<'a, MyFloat>,
    current: MyFloat,
}

impl<'a> Iterator for EnumerateValues<'a> {
    type Item = (MyFloat, &'a MyFloat);

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
    values: std::slice::IterMut<'a, MyFloat>,
    current: MyFloat,
}

impl<'a> Iterator for EnumerateValuesMut<'a> {
    type Item = (MyFloat, &'a mut MyFloat);

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

impl From<[MyFloat; consts::COUNT]> for Sampled {
    #[inline]
    fn from(arr: [MyFloat; consts::COUNT]) -> Self {
        Self(arr)
    }
}

impl<C> From<C> for Sampled
where
    C: Continuous
{
    fn from(spec: C) -> Self {
        Self::from_fn(|w| spec.evaluate(w))
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
