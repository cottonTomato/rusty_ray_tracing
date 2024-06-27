use crate::{interval::Interval, Vector3};
use std::fmt::Display;

#[derive(Default, Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from((r, g, b))
    }

    pub fn from_rgb_floats(r: f64, g: f64, b: f64) -> Self {
        Self::from((r, g, b))
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(value: (f64, f64, f64)) -> Self {
        static INTENSITY: Interval = Interval {
            min: 0.0,
            max: 0.999,
        };

        let rbyte = (256.0 * INTENSITY.clamp(value.0)) as u8;
        let gbyte = (256.0 * INTENSITY.clamp(value.1)) as u8;
        let bbyte = (256.0 * INTENSITY.clamp(value.2)) as u8;

        Self::from((rbyte, gbyte, bbyte))
    }
}

impl From<Vector3> for Color {
    fn from(value: crate::vec3::Vector3) -> Self {
        Self::from(value.to_floats())
    }
}
