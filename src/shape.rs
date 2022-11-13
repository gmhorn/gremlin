//! Shapes and surfaces
//!
//! To steal a definition from Mitsuba, a **shape** is a transition between
//! different types of materials. Depending on the complexity of the render, a
//! ray tracer will spend most of its time evaluating ray-object intersections
//! at shape boundaries.
//!
//! There are two main categories of shapes we need to deal with:
//! * Standalone primitives such as spheres and triangles
//! * Aggregations of primitives, such as triangle meshes.
//!
//! ## Vocabulary
//!
//! Naming things is hard, especially when it comes to

use crate::{geo::Ray, Float};

// RE-EXPORTS

mod aggregate;
pub use aggregate::*;

mod sphere;
pub use sphere::*;

mod surface;
pub use surface::*;

mod triangle;
pub use triangle::*;

// CORE DEFINITIONS

/// Encapsulates all information related to a ray-object intersection.
pub struct Intersection;

/// The core trait defining ray-object intersection.
///
/// This trait encapsulates the main functionality needed for efficient
/// ray-object intersection.
pub trait Shape {
    /// TODO: Delete me.
    fn value(&self) -> Float;

    /// Ray intersection test.
    ///
    /// Check whether the ray intersects this shape within the given
    /// `[t_min, t_max]` interval. Returns an [`Intersection`] record if so.
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection>;

    /// Fast ray intersection test.
    ///
    /// Primarily used for visibility queries. Simply returns a [`bool`]
    /// indicating whether or not the ray intersects this shape within the given
    /// `[t_min, t_max]` interval. Specifically does not return a detailed
    /// [`Intersection`] record.
    ///
    /// By default, this just forwards to [`intersect`]. For primitive shapes,
    /// this is usually the best we can do. For aggregate shapes like BVHs or
    /// kd-trees, it might be possible for implementers to optimize.
    ///
    /// [`intersect`]: Self::intersect
    #[inline]
    fn intersects(&self, ray: &Ray, t_min: Float, t_max: Float) -> bool {
        self.intersect(ray, t_min, t_max).is_some()
    }
}

pub enum Shapes {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Shape for Shapes {
    fn value(&self) -> Float {
        match self {
            Self::Sphere(sphere) => sphere.value(),
            Self::Triangle(triangle) => triangle.value(),
        }
    }

    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        todo!()
    }
}

impl From<Sphere> for Shapes {
    fn from(sphere: Sphere) -> Self {
        Shapes::Sphere(sphere)
    }
}

impl From<Triangle> for Shapes {
    fn from(triangle: Triangle) -> Self {
        Shapes::Triangle(triangle)
    }
}
