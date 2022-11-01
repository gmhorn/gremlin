/// Value is a tristimulus color value.
///
/// Doesn't have much meaning outside of the context of the color space its
/// defined in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Value(f32, f32, f32);

/// An individual film pixel which accumulates radiosity values.
///
/// Stores a running sum of the spectral sample contributions. Averaging by the
/// number of samples collected gives the final color value. Stores samples in a
/// tristimulus accumulator, rather than a full spectral accumulator. This is
/// valid to do, because
///
/// * averaging spectral values
/// * converting from a spectrum to a (linear) tristimulus colorspace point
/// * averaging (linear) tristimulus colorspace points
///
/// are all linear operations and distribute over each other. So we're free to
/// pick the most convenient order to perform them in. Converting to tristimulus
/// before storing and averaging saves substantial memory, at the cost of having
/// to perform the conversion more frequently.
///
/// See: <https://computergraphics.stackexchange.com/a/11000>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    value: Value,
    samples: u32,
}

impl Pixel {
    #[inline]
    pub const fn new() -> Self {
        Self {
            value: Value(0.0, 0.0, 0.0),
            samples: 0,
        }
    }
}
