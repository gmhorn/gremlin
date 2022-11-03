use crate::MyFloat;

use super::Sampled;

/// Trait for spectra that are continuous.
pub trait Continuous {
    /// Returns the value of the spectrum at the given wavelength.
    ///
    /// Wavelength units are in nanometers (*e.g.* `750.0` instead of `7.5e-7`).
    fn evaluate(&self, wavelength: MyFloat) -> MyFloat;

    /// Creates a sampled spectrum from this spectrum.
    #[inline]
    fn to_sampled(&self) -> Sampled {
        Sampled::from_fn(|w| self.evaluate(w))
    }
}

/// The spectrum around a black body of a given temperature.
pub struct Blackbody(MyFloat);

impl Blackbody {
    // https://en.wikipedia.org/wiki/Planck%27s_law#First_and_second_radiation_constants
    const C1: MyFloat = 3.74177e-16;
    const C2: MyFloat = 1.43879e-2;

    /// Creates a new blackbody spectrum for the given temperature (in Kelvin).
    ///
    /// Units are spectral radiant existance (power per unit area per unit
    /// wavelength).
    ///
    /// See also: <https://en.wikipedia.org/wiki/Planckian_locus>
    #[inline]
    pub const fn new(temp: MyFloat) -> Self {
        Self(temp)
    }
}

impl Continuous for Blackbody {
    fn evaluate(&self, wavelength: MyFloat) -> MyFloat {
        // Convert wavelength to meters
        let wavelength = wavelength * 1e-9;

        // Apply Plank's law
        let power_term = Self::C1 * wavelength.powi(-5);
        power_term / (Self::C2 / (wavelength * self.0)).exp_m1()
    }
}

/// A narrow Gaussian distribution centered at a given wavelength.
pub struct Peak {
    center: MyFloat,
    variance: MyFloat,
}

impl Peak {
    /// Creates a new peak distribution with the given center and variance.
    #[inline]
    pub fn new(center: MyFloat, variance: MyFloat) -> Self {
        Self { center, variance }
    }
}

impl Continuous for Peak {
    fn evaluate(&self, wavelength: MyFloat) -> MyFloat {
        1.0 / ((wavelength - self.center).powi(2) / (self.variance + self.variance)).exp()
    }
}

/// The refractive index through a medium.
pub struct Sellmeier {
    b_coeffs: [MyFloat; 3],
    c_coeffs: [MyFloat; 3],
}

impl Continuous for Sellmeier {
    fn evaluate(&self, wavelength: MyFloat) -> MyFloat {
        // Convert wavelengths to micrometers
        let wavelength = wavelength * 1e-3;
        // Precompute square
        let w_square = wavelength.powi(2);

        self.b_coeffs
            .iter()
            .zip(self.c_coeffs.iter())
            .fold(1.0, |n, (&b, &c)| n + (b * w_square) / (w_square - c))
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
