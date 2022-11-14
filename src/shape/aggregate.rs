use super::{Intersection, Shape};
use crate::{geo::Ray, Float};

pub type DirectAggregate<S> = Vec<S>;

impl<S: Shape> Shape for DirectAggregate<S> {
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        self.iter().fold(None, |curr, next| {
            let next = next.intersect(ray, t_min, t_max);
            match (curr, next) {
                (_, None) => curr,
                (None, _) => next,
                (Some(curr), Some(next)) => {
                    if curr.t < next.t {
                        Some(curr)
                    } else {
                        Some(next)
                    }
                }
            }
        })
    }
}

pub type DynamicAggregate = Vec<Box<dyn Shape>>;

impl Shape for DynamicAggregate {
    fn intersect(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<Intersection> {
        self.iter().fold(None, |curr, next| {
            let next = next.intersect(ray, t_min, t_max);
            match (curr, next) {
                (_, None) => curr,
                (None, _) => next,
                (Some(curr), Some(next)) => {
                    if curr.t < next.t {
                        Some(curr)
                    } else {
                        Some(next)
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{geo::Point, shape::Sphere};

    use super::*;

    #[test]
    fn dynamic_aggregate_add() {
        let mut agg = DynamicAggregate::new();
        let sphere = Sphere::new(Point::new(10.0, 0.0, 0.0), 1.0);
        agg.push(Box::new(sphere));
    }
}
