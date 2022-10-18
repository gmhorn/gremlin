#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit {
    x: f64,
    y: f64,
    z: f64,
}

impl Unit {
    pub const X_AXIS: Unit = Unit {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
}
