//! # Geometry module
//!
//! Implements basic geometric primitives needed for ray tracing. Attempts to
//! be simple, minimal, and fast (enough).
//!
//! * Uses minimal generics (most things parameterized over [`num_traits::Float`])
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

// MODULES AND RE-EXPORTS

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

// TYPE DEFINITIONS

pub type Mat32 = Matrix<f32>;
pub type Mat64 = Matrix<f64>;

pub type Pt32 = Point<f32>;
pub type Pt64 = Point<f64>;

pub type Ray32 = Ray<f32>;
pub type Ray64 = Ray<f64>;

pub type Unit32 = Unit<f32>;
pub type Unit64 = Unit<f64>;

pub type Vec32 = Vector<f32>;
pub type Vec64 = Vector<f64>;