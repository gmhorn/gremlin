//! Metrics.

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

/// A stopwatch for measuring elapsed time.
pub struct Timer(Instant);

impl Timer {
    /// Creates and starts a new timer.
    pub fn tick() -> Self {
        Self(Instant::now())
    }

    /// Fetches the duration that has elapsed since the timer started.
    pub fn tock(&self) -> Duration {
        Instant::now() - self.0
    }
}

/// An unsigned integer metric which only increments.
pub struct Counter(AtomicU64);

impl Counter {
    /// Create a new counter.
    pub const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    /// Increment the metric value by `1`.
    pub fn inc(&self) -> u64 {
        self.0.fetch_add(1, Ordering::Relaxed)
    }

    /// Retrieve the metric value.
    pub fn get(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
}

/// A [`f64`]-valued metric that can be incremented by arbitrary amounts.
pub struct Quantity(AtomicU64);

impl Quantity {
    /// Creates a new quantity.
    pub const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    /// Increment the metric value by `1.0`.
    pub fn inc(&self) -> f64 {
        self.inc_by(1.0)
    }

    /// Increment the metric value.
    pub fn inc_by(&self, v: f64) -> f64 {
        // Stolen pretty much directly from Prometheus's implementation.
        let mut old_u64 = self.0.load(Ordering::Relaxed);
        loop {
            let old_f64 = f64::from_bits(old_u64);
            let new_u64 = f64::to_bits(old_f64 + v);
            match self.0.compare_exchange_weak(
                old_u64,
                new_u64,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return old_f64,
                Err(x) => old_u64 = x,
            }
        }
    }

    /// Retrieve the metric value.
    pub fn get(&self) -> f64 {
        f64::from_bits(self.0.load(Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rayon::prelude::*;

    #[test]
    fn counter_inc() {
        let c = Counter::new();
        (0..1_000).into_par_iter().for_each(|_| {
            c.inc();
        });
        assert_eq!(1_000, c.get());
    }

    #[test]
    fn quantity_inc() {
        let q = Quantity::new();
        (0..1_000).into_par_iter().for_each(|_| {
            q.inc_by(0.1);
        });
        assert_relative_eq!(100.0, q.get(), epsilon = 1e-6);
    }
}
