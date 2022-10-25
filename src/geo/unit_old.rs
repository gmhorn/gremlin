/// Represents a unit vector, a vector with `len() == 1`.
///
/// There are no public constructors for unit vectors. These must be constructed
/// via the [`Vector::normalize()`] or [`Vector::try_normalize()`] methods.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitOld {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) z: f64,
}

impl UnitOld {
    pub const X_AXIS: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y_AXIS: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z_AXIS: Self = Self::new(0.0, 0.0, 1.0);

    #[inline]
    pub(super) const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
