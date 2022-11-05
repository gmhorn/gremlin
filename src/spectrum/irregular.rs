use crate::Float;

///
/// # Panics
///
/// Panics if any wavelengths are not comparable. IEEE floats don't exhibit a
/// total ordering due to `NaN` and whatnot. This assumes you've got
/// well-behaved floats and panics otherwise.
pub fn from_irreg(wavelengths: &[Float], values: &[Float]) {
    let mut sorted = wavelengths.iter().zip(values.iter()).collect::<Vec<_>>();
    sorted.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());
    let _ = sorted.iter().map(|a| a.0).collect::<Vec<_>>();
}
