//! Convenience re-export of common members.
//!
//! The purpose of this module is to alleviate imports of common structs and
//! traits by adding a glob import to the top of modules.
//!
//! ```
//! # #![allow(unused_imports)]
//! use gremlin::prelude::*;
//! ```

pub use crate::camera::Camera;
pub use crate::film::Save;
pub use crate::Float;
