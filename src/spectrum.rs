pub enum Wavelengths {
    Fixed,
}

pub struct FixedSampled<T, const N: usize> {
    data: [T; N],
}
