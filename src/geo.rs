//! # Geometric primitives.
//!
//! Implements basic geometric primitives needed for ray tracing. Attempts to
//! be simple, minimal, and fast (enough).
//!
//! * Uses minimal generics (underlying field is a compile-time flag, see
//!   [`Float`])
//! * Directly implements operator traits rather than relying on macros
//! * Supports the subset of functionality needed by other modules
//!
//! It does **not** attempt to be a fully-featured euclidean geometry library,
//! nor a fully-featured linear algebra library. There are already libraries for
//! that, such as:
//! * [`cgmath`](https://github.com/rustgd/cgmath) - Defines traits for general
//! linear-algebraic structures like vector spaces, inner-product spaces, normed
//! spaces, etc, and the implements them in generic structs.
//! * [`glam-rs`](https://github.com/bitshifter/glam-rs) - Fast float-valued
//! vector, matrix, quaterion and affine structures with SIMD implementations.
//! * [`nalgebra`](https://nalgebra.org) - A really impressive linear algebra
//! library for Rust
//! * [`ndarray`](https://github.com/rust-ndarray/ndarray) - An equally
//! impressive, Numpy-like N-dimensional array library.
//!
//! Instead, the goal is to be mathematically correct, while speaking in the
//! "domain language" of ray tracing. So, *e.g.* separate `Point`, `Vector` and
//! `Unit` structs are defined, even though the inner-product space of
//! float-valued vectors would be sufficient to cover all those use-cases. And
//! although `Point` is an inner-product space (with the standard operations of
//! vector addition and scalar multiplication), only the subset of traits from
//! `std::opts::*` that represent common "ray tracing operations on points" are
//! implemented on the `Point` struct. Additionality, despite the isomorphism
//! between the vector space of `Point`s and the vector space of `Vector`s,
//! ray tracing tends to use "homogeneous coordinates", where there's a
//! difference between point-like and vector-like matrix multiplication. Keeping
//! these as separate structs allows us to naturally express those differences
//! while still using the convenient operator overload for `*`.
//!
//! ## Algebra
//!
//! The primitives' operators obey the following algebra:
//!
//! ```text
//! Point - Point  = Vector
//! Point + Vector = Vector
//!
//! Scalar * Vector = Vector
//! Vector * Scalar = Vector
//! Vector / Scalar = Vector
//! Vector + Vector = Vector
//! Vector - Vector = Vector
//!
//! Scalar * Matrix = Matrix
//! Matrix * Scalar = Matrix
//! Matrix * Point  = Point
//! Matrix * Vector = Vector
//! Matrix * Unit   = Vector
//! Matrix * Matrix = Matrix
//! ```
//!
//! [`Float`]: crate::Float

// MODULES AND RE-EXPORTS

mod bounds;
pub use self::bounds::*;

mod coords;
pub use self::coords::*;

mod matrix;
pub use self::matrix::*;

mod point;
pub use self::point::*;

mod ray;
pub use self::ray::*;

mod unit;
pub use self::unit::*;

mod vector;
pub use self::vector::*;

/// Used to identify a coordinate.
///
/// Most the basic geometric structs in this package use named public fields,
/// such as `x`, `y`, and `z` for [`Point`]s and [`Vector`]s. Sometimes it's
/// useful to be able to pull them out by-index. This field supports that.
///
/// # Examples
///
/// Basic indexing:
///
/// ```
/// use gremlin::geo::{Axis, Vector};
///
/// let v = Vector::new(1.0, 2.0, 3.0);
/// assert_eq!(1.0, v[Axis::X]);
/// assert_eq!(2.0, v[Axis::Y]);
/// assert_eq!(3.0, v[Axis::Z]);
/// ```
///
/// Performing operations over indexes:
///
/// ```
/// use gremlin::geo::{Axis, Point};
///
/// let p = Point::new(1.0, 2.0, 3.0);
/// let sum_of_coords = Axis::ALL.iter().map(|&axis| p[axis]).sum();
/// assert_eq!(6.0, sum_of_coords);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    /// List of all axes. Convenient for iteration.
    pub const ALL: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
}
