use crate::{
    geo::{Matrix, Point, Ray, Vector},
    Float,
};

use super::Camera;

pub struct Perspective {
    aspect_ratio: Float,
    tan_half_fov: Float,

    eye: Point,
    target: Point,
    cam_to_world: Matrix,
}

impl Perspective {
    pub fn new(aspect_ratio: Float, fov: Float) -> Self {
        let tan_half_fov = (fov / 2.0).to_radians().tan();

        let eye = Point::ORIGIN;
        let target = Point::new(0.0, 0.0, -1.0);
        let cam_to_world = Matrix::look_at(eye, target, Vector::Y_AXIS);

        Self {
            aspect_ratio,
            tan_half_fov,
            eye,
            target,
            cam_to_world,
        }
    }

    pub fn move_to(&mut self, x: Float, y: Float, z: Float) {
        self.eye = Point::new(x, y, z);
        self.cam_to_world = Matrix::look_at(self.eye, self.target, Vector::Y_AXIS);
    }

    pub fn look_at(&mut self, x: Float, y: Float, z: Float) {
        self.target = Point::new(x, y, z);
        self.cam_to_world = Matrix::look_at(self.eye, self.target, Vector::Y_AXIS);
    }
}

impl Camera for Perspective {
    fn ray(&self, u: Float, v: Float) -> Ray {
        // little shortcut. This is really the point on the screen, and the
        // direction would be given by subtracting the camera eye. But the
        // camera is set to the origin, so we can skip the subtraction and
        // use it straight-up as the direction vector.
        let screen_pt = Vector::new(
            (2.0 * u - 1.0) * self.aspect_ratio * self.tan_half_fov,
            (1.0 - 2.0 * v) * self.tan_half_fov,
            -1.0,
        );
        Ray::new(self.eye, self.cam_to_world * screen_pt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let mut cam = Perspective::new(1.0, 1.0);
        cam.look_at(1.0, 2.0, 3.0);

        let _r = cam.ray(1.0, 1.0);
    }
}
