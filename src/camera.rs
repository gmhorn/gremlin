//! Camera models.
//!
//! Cameras generate rays through a pixel. This is a major component of the
//! top-level rendering loop. Generally speaking, camera models often require
//! a large number of parameters to fully specify. To make this simpler, we use
//! the builder pattern to enable calling code to only specify the parameters
//! needed.
//!
//! ```
//! use gremlin::camera::ThinLens
//! use gremlin::prelude::*;
//!
//! let resolution = (800, 600);
//! let cam = ThinLens::builder(resolution)
//!     .move_to([0.0, 0.0, -10.0])
//!     .aperture(0.1)
//!     .auto_focus()
//!     .build();
//! ```
use crate::{
    geo::{Matrix, Point, Ray, Vector},
    Float,
};
use rand::prelude::*;
use rand_distr::UnitDisc;

const DEFAULT_LOOK_FROM: Point = Point::new(0.0, 0.0, -1.0);
const DEFAULT_LOOK_AT: Point = Point::ORIGIN;
const DEFAULT_FOV: Float = 75.0;

/// The core trait for objects which generate rays.
pub trait Camera: Send + Sync {
    /// Generate a ray for the pixel at coordinates `(px, py)`.
    fn ray(&self, px: u32, py: u32, rng: &mut impl Rng) -> Ray;
}

/// An idealized thin-lens camera.
#[derive(Debug, Clone)]
pub struct ThinLens {
    resolution_width: Float,
    resolution_height: Float,
    aspect_ratio: Float,
    tan_half_fov: Float,
    focus_distance: Float,
    half_aperture: Float,
    cam_to_world: Matrix,
}

impl ThinLens {
    /// Create a new thin lens camera builder with the given resolution.
    ///
    /// See [`ThinLensBuilder::new`] for details.
    pub fn builder((width, height): (u32, u32)) -> ThinLensBuilder {
        ThinLensBuilder::new(width, height)
    }
}

impl Camera for ThinLens {
    fn ray(&self, px: u32, py: u32, rng: &mut impl Rng) -> Ray {
        // Pick a random point in pixel and convert to NDC space
        let u = ((px as Float) + rng.gen::<Float>()) / self.resolution_width;
        let v = ((py as Float) + rng.gen::<Float>()) / self.resolution_height;

        // Express that "random point in the pixel"'s location in screen space
        let screen_pt = Vector {
            x: (2.0 * u - 1.0) * self.aspect_ratio * self.tan_half_fov,
            y: (1.0 - 2.0 * v) * self.tan_half_fov,
            z: -1.0,
        };

        // Project it into the focal plane. Since our camera origin is at
        // the coordinate space origin, this is simply scaling by the focal
        // distance
        let focal_pt = screen_pt * self.focus_distance;

        // The ray originates from a random point in the unit disk, centered at
        // the origin and scaled by the aperture size
        let rand_in_disc: [Float; 2] = UnitDisc.sample(rng);
        let origin_pt = Vector::new(rand_in_disc[0], rand_in_disc[1], 0.0) * self.half_aperture;

        // This is our final ray, in camera space
        let ray = Ray::new(origin_pt.into(), focal_pt - origin_pt);

        // The is our ray in world space
        self.cam_to_world * ray
    }
}

/// Builder for creating [`ThinLens`] camera instances.
pub struct ThinLensBuilder {
    look_from: Point,
    look_at: Point,
    inner: ThinLens,
}

impl ThinLensBuilder {
    /// Create a new thin lens camera builder with the given resolution.
    ///
    /// By default, the camera will be placed at `(0, 0, -1)`, looking at the
    /// origin. It starts off with an zero-sized aperture (all points of the
    /// scene will be in focus).
    pub fn new(width: u32, height: u32) -> Self {
        let resolution_width = width as Float;
        let resolution_height = height as Float;
        let aspect_ratio = resolution_width / resolution_height;

        let mut builder = Self {
            look_from: DEFAULT_LOOK_FROM,
            look_at: DEFAULT_LOOK_AT,
            inner: ThinLens {
                resolution_width,
                resolution_height,
                aspect_ratio,
                half_aperture: 0.0,
                focus_distance: 1.0,
                tan_half_fov: 0.5,              // temporary!
                cam_to_world: Matrix::IDENTITY, // temporary!
            },
        };

        builder.fov(DEFAULT_FOV);
        builder.recalculate_look_matrix();
        builder
    }

    /// Move the camera to a new location.
    pub fn move_to(&mut self, eye: impl Into<Point>) -> &mut Self {
        self.look_from = eye.into();
        self.recalculate_look_matrix();
        self
    }

    /// Point the camera at a new location.
    pub fn look_at(&mut self, target: impl Into<Point>) -> &mut Self {
        self.look_at = target.into();
        self.recalculate_look_matrix();
        self
    }

    /// Set the field-of-view, in degrees.
    pub fn fov(&mut self, fov: Float) -> &mut Self {
        self.inner.tan_half_fov = (fov / 2.0).to_radians().tan();
        self
    }

    /// Set the aperture.
    pub fn aperture(&mut self, aperture: Float) -> &mut Self {
        self.inner.half_aperture = aperture * 0.5;
        self
    }

    /// Set the focal length.
    pub fn focal_length(&mut self, len: Float) -> &mut Self {
        self.inner.focus_distance = len;
        self
    }

    /// Set the focal length so that the [`look_at`] point is in-focus.
    ///
    /// [`look_at`]: Self::look_at
    pub fn auto_focus(&mut self) -> &mut Self {
        self.inner.focus_distance = (self.look_at - self.look_from).len();
        self
    }

    /// Creates a new thin lens camera from this builder.
    pub fn build(&self) -> ThinLens {
        self.inner.clone()
    }

    fn recalculate_look_matrix(&mut self) {
        self.inner.cam_to_world = Matrix::look_at(self.look_from, self.look_at, Vector::Y_AXIS);
    }
}
