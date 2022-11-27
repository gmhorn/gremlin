//! Camera models.
//!
//! The purpose of a camera is, given pixel, generate rays through that pixel.
//! Generally many ray samples will be generated for each pixel.
use crate::{
    geo::{Matrix, Point, Ray, Vector},
    Float,
};
use rand::prelude::*;
use rand_distr::UnitDisc;

const DEFAULT_LOOK_FROM: Point = Point::new(0.0, 0.0, -1.0);
const DEFAULT_LOOK_AT: Point = Point::ORIGIN;
const DEFAULT_FOV: Float = 75.0;
const DEFAULT_APERTURE: Float = 0.1;

/// The core trait for objects which generate rays.
pub trait Camera {
    fn ray(&self, u: Float, v: Float) -> Ray;
}

/// An idealized pinhole camera.
#[derive(Debug, Clone)]
pub struct Pinhole {
    aspect_ratio: Float,
    tan_half_fov: Float,
    cam_to_world: Matrix,
}

impl Pinhole {
    /// Create a new pinhole camera builder with the given aspect ratio.
    pub fn builder(aspect_ratio: Float) -> PinholeBuilder {
        PinholeBuilder::new(aspect_ratio)
    }
}

impl Camera for Pinhole {
    fn ray(&self, u: Float, v: Float) -> Ray {
        // Small shortcut. As the name suggests, screen_pt is the screen-space
        // point corresponding to the (u, v) NDC-space coordinates. So to get
        // a proper vector, we'd need to subtract this with the camera's origin
        // point.
        //
        // But since, by convention, the camera is at the origin and the screen
        // at z = -1 (in camera space), we can skip the subtraction and use this
        // directly as a vector.
        let screen_pt = Vector {
            x: (2.0 * u - 1.0) * self.aspect_ratio * self.tan_half_fov,
            y: (1.0 - 2.0 * v) * self.tan_half_fov,
            z: -1.0,
        };
        // This is the ray in camera space, so convert it to world-space
        let ray = Ray::new(Point::ORIGIN, screen_pt);
        self.cam_to_world * ray
    }
}

/// A pinhole camera builder.
pub struct PinholeBuilder {
    look_from: Point,
    look_at: Point,
    inner: Pinhole,
}

impl PinholeBuilder {
    /// Create a new pinhole camera builder with the given aspect ratio.
    pub fn new(aspect_ratio: Float) -> Self {
        let mut builder = Self {
            look_from: DEFAULT_LOOK_FROM,
            look_at: DEFAULT_LOOK_AT,
            inner: Pinhole {
                aspect_ratio: aspect_ratio,
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

    /// Creates a new pinhole camera from this builder.
    pub fn build(&self) -> Pinhole {
        self.inner.clone()
    }

    fn recalculate_look_matrix(&mut self) {
        self.inner.cam_to_world = Matrix::look_at(self.look_from, self.look_at, Vector::Y_AXIS);
    }
}

#[derive(Debug, Clone)]
pub struct ThinLens {
    aspect_ratio: Float,
    tan_half_fov: Float,
    focus_distance: Float,
    aperture: Float,
    cam_to_world: Matrix,
}

impl ThinLens {
    /// Create a new thin lens camera builder with the given aspect ratio.
    pub fn builder(aspect_ratio: Float) -> ThinLensBuilder {
        ThinLensBuilder::new(aspect_ratio)
    }
}

impl Camera for ThinLens {
    fn ray(&self, u: Float, v: Float) -> Ray {
        let screen_pt = Point {
            x: (2.0 * u - 1.0) * self.focus_distance * self.aspect_ratio * self.tan_half_fov,
            y: (1.0 - 2.0 * v) * self.focus_distance * self.tan_half_fov,
            z: -self.focus_distance,
        };
        let rand_in_disc: [Float; 2] = UnitDisc.sample(&mut thread_rng());
        let origin = Point {
            x: self.aperture * 0.5 * rand_in_disc[0],
            y: self.aperture * 0.5 * rand_in_disc[1],
            z: 0.0,
        };
        let ray = Ray::new(origin, screen_pt - origin);
        self.cam_to_world * ray
    }
}

pub struct ThinLensBuilder {
    look_from: Point,
    look_at: Point,
    inner: ThinLens,
}

impl ThinLensBuilder {
    /// Create a new thin lens camera builder with the given aspect ratio.
    pub fn new(aspect_ratio: Float) -> Self {
        let mut builder = Self {
            look_from: DEFAULT_LOOK_FROM,
            look_at: DEFAULT_LOOK_AT,
            inner: ThinLens {
                aspect_ratio: aspect_ratio,
                tan_half_fov: 0.5,   // temporary!
                focus_distance: 1.0, // temporary!
                aperture: DEFAULT_APERTURE,
                cam_to_world: Matrix::IDENTITY, // temporary!
            },
        };

        builder.fov(DEFAULT_FOV);
        builder.auto_focus();
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
        self.inner.aperture = aperture;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinhole_builder() {
        let cam = Pinhole::builder(1.6).move_to([0.0, 0.0, -2.0]).build();
        println!("{:?}", cam);
    }
}
