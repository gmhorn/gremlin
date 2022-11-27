use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::ops::{Add, Div, Mul, Neg, Sub};

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

// OPERATORS

impl<T: Neg<Output = T>> Neg for Coords<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(self.x.neg(), self.y.neg())
    }
}

impl<T: Add<Output = T>> Add for Coords<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Coords<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Coords<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Div<T, Output = T> + Copy> Div<T> for Coords<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs)
    }
}

// APPROXIMATIONS

impl<T: AbsDiffEq> AbsDiffEq for Coords<T>
where
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[rustfmt::skip]
    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.x, &other.x, epsilon) &&
        T::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

impl<T: RelativeEq> RelativeEq for Coords<T>
where
    T::Epsilon: Copy,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[rustfmt::skip]
    #[inline]
    fn relative_eq(
        &self,
        other: &Self, 
        epsilon: Self::Epsilon, 
        max_relative: Self::Epsilon,
    ) -> bool {
        T::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        T::relative_eq(&self.y, &other.y, epsilon, max_relative)
    }
}

impl<T: UlpsEq> UlpsEq for Coords<T>
where
    T::Epsilon: Copy,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[rustfmt::skip]
    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        T::ulps_eq(&self.x, &other.x, epsilon, max_ulps) &&
        T::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
    }
}

// CONVERSIONS: COORDS -> OTHER

impl<T: Copy> From<Coords<T>> for [T; 2] {
    #[inline]
    fn from(coords: Coords<T>) -> Self {
        [coords.x, coords.y]
    }
}

impl<T: Copy> From<Coords<T>> for (T, T) {
    #[inline]
    fn from(coords: Coords<T>) -> Self {
        (coords.x, coords.y)
    }
}

// CONVERSIONS: OTHER -> COORDS

impl<T: Copy> From<[T; 2]> for Coords<T> {
    #[inline]
    fn from(arr: [T; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }
}

impl<T: Copy> From<(T, T)> for Coords<T> {
    #[inline]
    fn from(tuple: (T, T)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}
