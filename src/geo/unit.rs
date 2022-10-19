/// Represents a unit vector, a vector with `len() == 1`.
///
/// There are no public constructors for unit vectors. The easiest way is to
/// call the `normalize()` method on a vector instead.
///
/// ```
/// use gremlin::geo::Vector;
///
/// let v = Vector::new(2.0, 3.0, 4.0);
/// let u = v.normalize().expect("should be nonzero");
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) z: f64,
}

impl Unit {
    pub const X_AXIS: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y_AXIS: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z_AXIS: Self = Self::new(0.0, 0.0, 1.0);

    pub(super) const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
