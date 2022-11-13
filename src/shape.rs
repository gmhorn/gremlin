use crate::Float;

pub trait ShapeDefinition {
    fn value(&self) -> Float;
}

pub enum Shape {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl ShapeDefinition for Shape {
    fn value(&self) -> Float {
        match self {
            Shape::Sphere(sphere) => sphere.value(),
            Shape::Triangle(triangle) => triangle.value(),
        }
    }
}

pub struct Sphere;

impl ShapeDefinition for Sphere {
    fn value(&self) -> Float {
        2.0
    }
}

impl From<Sphere> for Shape {
    fn from(sphere: Sphere) -> Self {
        Shape::Sphere(sphere)
    }
}

pub struct Triangle;

impl ShapeDefinition for Triangle {
    fn value(&self) -> Float {
        1.0
    }
}

impl From<Triangle> for Shape {
    fn from(triangle: Triangle) -> Self {
        Shape::Triangle(triangle)
    }
}