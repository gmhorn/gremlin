use crate::Float;

/// The spectral radiant existance of a black body at a given temperature.
///
/// Temperature is given in Kelvins and wavelength in nanometers. Returned
/// units are power per unit area per unit wavelength.
///
/// # Examples
///
/// ```
/// use gremlin::spectrum::{self, Sampled};
///
/// let temp = 6500.0;
/// let _ = Sampled::from(|w| spectrum::blackbody(temp, w));
/// ```
///
/// See also: <https://en.wikipedia.org/wiki/Planckian_locus>
pub fn blackbody(temp: Float, wavelength: Float) -> Float {
    // https://en.wikipedia.org/wiki/Planck%27s_law#First_and_second_radiation_constants
    const C1: Float = 3.74177e-16;
    const C2: Float = 1.43879e-2;

    // Convert wavelength to meters
    let wavelength = wavelength * 1e-9;

    // Apply Plank's law
    let power_term = C1 * wavelength.powi(-5);
    power_term / (C2 / (wavelength * temp)).exp_m1()
}

/// A the value of a guassian spectrum with a given mean and variance.
///
/// # Examples
///
/// ```
/// use gremlin::spectrum::{self, Sampled};
///
/// let temp = 6500.0;
/// let _ = Sampled::from(|w| spectrum::gaussian(550.0, 15.0, w));
/// ```
pub fn gaussian(mean: Float, variance: Float, wavelength: Float) -> Float {
    1.0 / ((wavelength - mean).powi(2) / (2.0 * variance)).exp()
}

/// The refractive index through a medium.
pub fn sellmeier(bs: &[Float; 3], cs: &[Float; 3], wavelength: Float) -> Float {
    // Convert wavelength to micrometers
    let wavelength = wavelength * 1e-3;
    // Precompute square
    let w_square = wavelength.powi(2);

    bs.iter()
        .zip(cs.iter())
        .fold(1.0, |n, (&b, &c)| n + (b * w_square) / (w_square - c))
}
