#[derive(Debug, PartialEq, Clone)]
pub struct FloatSample([f32; 80]);

impl FloatSample {
    pub const fn splat(n: f32) -> Self {
        Self([n; 80])
    }

    pub fn add_loop(&self, other: &Self) -> Self {
        let mut res = [0.0; 80];

        for i in 0..80 {
            res[i] = self.0[i] + other.0[i];
        }

        Self(res)
    }

    pub fn add_iter(&self, other: &Self) -> Self {
        Self(self.0.iter().zip(other.0)
            .map(|(a, b)| a + b)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let _s = FloatSample::splat(1.0);
    }
}