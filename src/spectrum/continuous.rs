use crate::Real;

use super::Sampled;

/// Trait for spectra that are continuous.
pub trait Continuous<R: Real> {
    /// Returns the value of the spectrum at the given wavelength.
    ///
    /// Wavelength units are in nanometers (*e.g.* `750.0` instead of `7.5e-7`).
    fn evaluate(&self, wavelength: R) -> R;

    /// Creates a sampled spectrum from this spectrum.
    #[inline]
    fn to_sampled(&self) -> Sampled<R> {
        Sampled::<R>::from_fn(|w| self.evaluate(w))
    }
}

/// The spectrum around a black body of a given temperature.
pub struct Blackbody<R>(R);

impl<R: Real> Blackbody<R> {
    /// Creates a new blackbody spectrum for the given temperature (in Kelvin).
    ///
    /// Units are spectral radiant existance (power per unit area per unit
    /// wavelength).
    ///
    /// See also: <https://en.wikipedia.org/wiki/Planckian_locus>
    #[inline]
    pub const fn new(temp: R) -> Self {
        Self(temp)
    }
}

impl<R: Real> Continuous<R> for Blackbody<R> {
    fn evaluate(&self, wavelength: R) -> R {
        // https://en.wikipedia.org/wiki/Planck%27s_law#First_and_second_radiation_constants
        let c1 = R::from_f32(3.74177e-16);
        let c2 = R::from_f32(1.43879e-2);

        // Convert wavelength to meters
        let wavelength = wavelength * 1e-9_f32.into();

        // Apply Plank's law
        let power_term = c1 * wavelength.powi(-5);
        power_term / (c2 / (wavelength * self.0)).exp_m1()
    }
}

/// A narrow Gaussian distribution centered at a given wavelength.
pub struct Peak<R> {
    center: R,
    variance: R,
}

impl<R: Real> Peak<R> {
    /// Creates a new peak distribution with the given center and variance.
    #[inline]
    pub fn new(center: R, variance: R) -> Self {
        Self { center, variance }
    }
}

impl<R: Real> Continuous<R> for Peak<R> {
    fn evaluate(&self, wavelength: R) -> R {
        R::one() / ((wavelength - self.center).powi(2) / (self.variance + self.variance)).exp()
    }
}

/// The refractive index through a medium.
pub struct Sellmeier<R> {
    b_coeffs: [R; 3],
    c_coeffs: [R; 3],
}

impl<R: Real> Continuous<R> for Sellmeier<R> {
    fn evaluate(&self, wavelength: R) -> R {
        // Convert wavelengths to micrometers
        let wavelength = wavelength * R::from_f32(1e-3);
        // Precompute square
        let w_square = wavelength.powi(2);

        self.b_coeffs
            .iter()
            .zip(self.c_coeffs.iter())
            .fold(R::one(), |n, (&b, &c)| n + (b * w_square) / (w_square - c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peak() {
        let p = Peak::new(500.0, 15.0).to_sampled();
        for (wl, &value) in p.enumerate_values() {
            println!("{}: {}", wl, value);
        }
    }
}
