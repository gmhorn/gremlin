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
    pub fn new<T>(xs: T, ys: T) -> Self
    where
        T: AsRef<[Float]>,
    {
        let xs = xs.as_ref().iter();
        let ys = ys.as_ref().iter();

        let mut coords: Vec<_> = xs.zip(ys).map(|(&x, &y)| Coord(x, y)).collect();
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
    pub fn y(&self, x: Float) -> Float {
        // match self.binary_search(x) {
        //     Ok(idx) => self.coords[idx].1,
        //     Err(idx) => {
        //         let idx2 = 
        //     }
        // }
        todo!()
    }

    #[inline(always)]
    fn binary_search(&self, x: Float) -> Result<usize, usize> {
        self.coords.binary_search_by(|c| c.0.total_cmp(&x))
    }

    /// Calculates the average value of the function between `x0` and `x1`.
    pub fn average_value(&self, x0: Float, x1: Float) -> Float {
        self.integrate(x0, x1) / (x1 - x0).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let f = PiecewiseLinearFn::new([-1.0, 0.0, 1.0], [1.0, 0.0, 1.0]);

        // assert_eq!(2.0, f.y(-2.0)); // Outside domain, negative
        // assert_eq!(0.5, f.y(-0.5)); // Inside domain
        assert_eq!(0.0, f.y(0.0));  // Exact value
        // assert_eq!(0.5, f.y(0.5));  // Inside domain
        // assert_eq!(2.0, f.y(2.0));  // Outside domain, positive
    }
}