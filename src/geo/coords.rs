/// Represents a 2-dimensional point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coords<T> {
    /// Constructs coordinates from the given `(x, y)` pair.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Coords<T> {
    /// Constructs coordinates with `x == y == n`
    #[inline]
    pub const fn splat(n: T) -> Self {
        Self::new(n, n)
    }
}
