//! # Spectrum module.
//!
//! A spectral distribution is a quantity that is a function of wavelength.
//! Examples are reflectance, refractive index, radiance, etc.
//!
//! The core data type is [`Sampled`], which is a spectrum defined at fixed,
//! uniformly-spaced sample wavelengths. For our purposes, these are the
//! human-visible wavelengths, roughly 380-780nm.
//!
//! [`Sampled`] is designed to as efficient as possible, in terms of both space
//! and performance. It is stack-allocated and supports efficient iteration,
//! construction, and mathematical operations.
//!
//! This package also defines some common continuous spectra, implemented as
//! regular functions. Typically these are expensive to compute, so are only
//! usable when converted into a [`Sampled`] instance.
//!
//! ```
//! use gremlin::spectrum::{self, Sampled};
//!
//! let temp = 6500.0;
//! let blackbody_spectrum = Sampled::from(|w| spectrum::blackbody(temp, w));
//! // do something with blackbody_spectrum
//! ```

mod continuous;
pub use continuous::*;

mod sampled;
pub use sampled::*;
