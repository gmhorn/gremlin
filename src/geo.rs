//! # Geometry module
//!
//! Implements basic geometric primitives needed for ray tracing. Attempts to
//! be simple, minimal, and fast (enough).
//!
//! * Uses minimal generics (`f64` is sufficient most of the time)
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
//!
//! Instead, the goal is to be mathematically correct, while speaking in the
//! "domain language" of ray tracing. So, *e.g.* separate `Point`, `Vector` and
//! `Unit` structs are defined, even though the inner-product space of
//! `f64`-valued vectors would be sufficient to cover all those use-cases. And
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
//! Point - Point = Vector
//! Point + Vector = Vector
//!
//! Vector * Scalar = Vector
//! Vector / Scalar = Vector
//! Vector + Vector = Vector
//! Vector - Vector = Vector
//!
//! Matrix * Point = Point
//! Matrix * Vector = Vector
//! Matrix * Unit = Vector
//! Matrix * Matrix = Matrix
//! ```

/* Modules and re-exports */

mod coords;
pub use self::coords::*;

mod matrix;
pub use self::matrix::*;

mod mtx4;
pub use self::mtx4::*;

mod point_old;
pub use self::point_old::*;

mod point;
pub use self::point::*;

mod ray;
pub use self::ray::*;

mod unit;
pub use self::unit::*;

mod vector;
pub use self::vector::*;

mod vec3;
pub use self::vec3::*;

/* Convenience functions */

/// Constructs coordinates from the given `(x, y)` pair.
#[inline]
pub const fn coords<T>(x: T, y: T) -> Coords<T> {
    Coords::new(x, y)
}

/// Creates a new point.
#[inline]
pub const fn point(x: f64, y: f64, z: f64) -> PointOld {
    PointOld::new(x, y, z)
}

/// Creates a new vector
#[inline]
pub const fn vector(x: f64, y: f64, z: f64) -> Vector {
    Vector::new(x, y, z)
}
