use super::Vector;
use crate::MyFloat;
use std::ops::Neg;

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
pub struct Unit {
    x: MyFloat,
    y: MyFloat,
    z: MyFloat,
}

impl Unit {

    /// The unit vector along the x-axis.
    pub const X_AXIS: Unit = Unit::new(1.0, 0.0, 0.0);

    /// The unit vector along the y-axis.
    pub const Y_AXIS: Unit = Unit::new(0.0, 1.0, 0.0);

    /// The unit vector along the z-axis.
    pub const Z_AXIS: Unit = Unit::new(0.0, 0.0, 1.0);

    #[inline]
    const fn new(x: MyFloat, y: MyFloat, z: MyFloat) -> Self {
        Self { x, y, z }
    }
}

// OPERATORS

impl Neg for Unit {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
}

// CONVERSIONS: UNIT -> OTHER

impl From<Unit> for [MyFloat; 3] {
    #[inline]
    fn from(u: Unit) -> Self {
        [u.x, u.y, u.z]
    }
}

impl From<Unit> for Vector {
    #[inline]
    fn from(u: Unit) -> Self {
        Self::new(u.x, u.y, u.z)
    }
}

// CONVERSIONS: OTHER -> UNIT

impl TryFrom<Vector> for Unit {
    type Error = &'static str;

    #[inline]
    fn try_from(v: Vector) -> Result<Self, Self::Error> {
        let recip = v.len().recip();
        match recip.is_normal() {
            true => Ok(Self::new(v.x * recip, v.y * recip, v.z * recip)),
            false => Err("Cannot normalize zero-length or ill-conditioned vectors"),
        }
    }
}
