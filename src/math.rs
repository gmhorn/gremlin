use crate::Float;

// Small tuple struct to hold `(x, y)` values.
#[derive(Debug, Clone, Copy)]
struct Coord(Float, Float);

/// A piecewise-linear function.
///
/// There's a couple places where it's convenient to have a halfway-decent
/// toolkit for handling piecewise-linear functions. This is not the most
/// efficient, general-purpose implementation one could imagine. But it's
/// good enough.
///
/// Note that this implementation assumes "compact support". That is, the value
/// is assumed to be 0 outside of the domain. There's no real a-priori
/// justification for this. But most of the time we're not interested in values
/// outside the domain, and assuming a value of 0 makes things slightly more
/// convenient.
#[derive(Debug, Clone)]
pub struct PiecewiseLinearFn {
    coords: Vec<Coord>,
}

impl PiecewiseLinearFn {
    /// Constructs a new piecewise-linear function with the given x and y values.
    ///
    /// Allocates on the heap.
    pub fn new(xs: &[Float], ys: &[Float]) -> Self {
        let mut coords: Vec<_> = xs
            .iter()
            .zip(ys.iter())
            .map(|(&x, &y)| Coord(x, y))
            .collect();
        coords.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        Self { coords }
    }

    /// Integrates the function between `x0` and `x1`.
    pub fn integrate(&self, x0: Float, x1: Float) -> Float {
        let c0 = self.coords.binary_search_by(|v| v.0.total_cmp(&x0));
        let c1 = self.coords.binary_search_by(|v| v.0.total_cmp(&x1));
        todo!()
    }

    /// Evaluates the function at the given x-value.
    pub fn y(x: Float) -> Float {
        todo!()
    }

    /// Calculates the average value of the function between `x0` and `x1`.
    pub fn average_value(&self, x0: Float, x1: Float) -> Float {
        self.integrate(x0, x1) / (x1 - x0).abs()
    }
}
