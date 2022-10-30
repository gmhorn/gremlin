use num_traits::Float;

pub struct Wavelengths<F> {
    min: F,
    max: F,
    step: F,
}

impl<F: Float> Wavelengths<F> {
    pub const fn new(min: F, max: F, step: F) -> Self {
        Self{ min, max, step }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FloatSample([f32; 80]);
impl FloatSample {
    pub const fn splat(n: f32) -> Self {
        Self([n; 80])
    }

    pub fn add_iter(&self, other: &Self) -> Self {
        let mut data = self.0;

        for (idx, val) in data.iter_mut().enumerate() {
            *val += other.0[idx];
        }

        Self(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let foo = [0.0; 80];

        let _s = FloatSample::splat(1.0);
    }
}