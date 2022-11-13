use crate::Float;

pub trait Shape {
    fn value(&self) -> Float;
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
}

pub struct Sphere;

impl Shape for Sphere {
    fn value(&self) -> Float {
        2.0
    }
}

impl From<Sphere> for Shapes {
    fn from(sphere: Sphere) -> Self {
        Shapes::Sphere(sphere)
    }
}

pub struct Triangle;

impl Shape for Triangle {
    fn value(&self) -> Float {
        1.0
    }
}

impl From<Triangle> for Shapes {
    fn from(triangle: Triangle) -> Self {
        Shapes::Triangle(triangle)
    }
}