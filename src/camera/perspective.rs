use crate::{Float, geo::{Point, Matrix, Ray, Vector}};

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

        Self {aspect_ratio, tan_half_fov, eye, target, cam_to_world}
    }

    pub fn move_to(&mut self, location: Point) {
        self.eye = location;
        self.cam_to_world = Matrix::look_at(self.eye, self.target, Vector::Y_AXIS);
    }
    
    pub fn look_at(&mut self, location: Point) {
        self.target = location;
        self.cam_to_world = Matrix::look_at(self.eye, self.target, Vector::Y_AXIS);
    }
}

impl Camera for Perspective {
    fn ray(&self, u: Float, v: Float) -> Ray {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn foo() {
        let mut cam = Perspective::new(1.0, 1.0);
        cam.move_to(Point::new(1.0, 2.0, 3.0));
        cam.look_at(Point::new(1.0, 2.0, 3.0));

        let r = cam.ray(1.0, 1.0);
    }
}