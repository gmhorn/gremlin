use std::ops::Neg;

use num_traits::Float;

use super::Vector;

/// A 3-dimensional unit vector.
/// 
/// We enforce a separate type because whether or not a vector is normalized may
/// be significant in some rendering operations. For similar reasons, there is
/// no public constructor that takes arbitrary component values. A fixed set of
/// known-valid constructors, and conversions such as [`Vector::normalize`] and
/// the [`TryFrom`] trait are the only way to construct these types.
/// 
/// Very few operators and methods are implemented on [`Unit`]. That's because,
/// generally speaking, operations such as:
/// 
/// * Multiplying or dividing by a scalar
/// * Adding or subtracting by another [`Unit`] or [`Vector`]
/// * Transforming by a [`Matrix`][super::Matrix]
/// 
/// are not length-preserving. Similarly, the `(x, y, z)` fields are private,
/// since exposing them would allow code to violate the unit-length invariant.
/// Consequently, [`Vector`]s are much easier to work with in practice.
/// 
/// Its tempting to minimize the number of APIs that take [`Unit`] arguments. 
/// The trade-off with taking [`Vector`] arguments everywhere and internally
/// converting is:
/// * There is a cost to the conversion, so it matters if the API is expected to
///   be used on hot code paths.
/// * Conversion may fail for arbitrary [`Vector`]s. So APIs need to either
///   expose that possibility in their signatures, or panic.
/// 
/// Panicing may be especially annoying if conversion fails many minutes in to a
/// long render. So like everything, it's a trade-off.
pub struct Unit<F> {
    x: F,
    y: F,
    z: F,
}

impl<F: Float> Unit<F> {

    #[inline]
    const fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

    /// Construct a new unit vector along the x-axis.
    #[inline]
    pub fn x_axis() -> Self {
        Self::new(F::one(), F::zero(), F::zero())
    }

    /// Construct a new unit vector along the y-axis
    #[inline]
    pub fn y_axis() -> Self {
        Self::new(F::zero(), F::one(), F::zero())
    }

    /// Construct a new unit vector along the z-axis
    #[inline]
    pub fn z_axis() -> Self {
        Self::new(F::zero(), F::zero(), F::one())
    }
}

// OPERATORS

impl<F: Float> Neg for Unit<F> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}

// CONVERSIONS: UNIT -> OTHER

impl<F: Float> From<Unit<F>> for [F; 3] {
    #[inline]
    fn from(u: Unit<F>) -> Self {
        [u.x, u.y, u.z]
    }
}

impl<F: Float> From<Unit<F>> for Vector<F> {
    #[inline]
    fn from(u: Unit<F>) -> Self {
        Self::new(u.x, u.y, u.z)
    }
}

// CONVERSIONS: OTHER -> UNIT

impl<F: Float> TryFrom<Vector<F>> for Unit<F> {
    type Error = &'static str;

    #[inline]
    fn try_from(v: Vector<F>) -> Result<Self, Self::Error> {
        let recip = v.len().recip();
        match recip.is_normal() {
            true => Ok(Self::new(v.x * recip, v.y * recip, v.z * recip )),
            false => Err("Cannot normalize zero-length or ill-conditioned vectors"),
        }
    }
}