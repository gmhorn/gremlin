//! # Geometry module
//!
//! Implements basic geometric primitives needed for ray tracing. Attempts to
//! be simple, minimal, and fast.
//!
//! * Uses minimal generics (`f64` is sufficient most of the time)
//! * Directly implements operator traits rather than relying on macros
//! * Supports the minimum subset of functionality needed by other modules
//!
//! It does **not** attempt to be a fully-featured euclidean geometry library,
//! nor a fully-featured linear algebra library.

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
