//! # Spectrum module.
//! 
//! A spectral distribution is a quantity that is a function of wavelength.
//! Examples are reflectance, refractive index, radiance, etc. This module
//! defines two core types:
//! 
//! * [`Sampled`], which is used for spectra that are defined at fixed sample
//!   wavelengths.
//! * [`Continuous`], which are spectra that are defined for a continuous range
//!   of wavelengths.
//! 
//! The wavelengths we care about are mostly in the human-visible range, roughly
//! 380nm to 780nm. As a result, most wavelength units are in nanometers rather
//! than meters.

mod continuous;
pub use continuous::*;

mod sampled;
pub use sampled::*;