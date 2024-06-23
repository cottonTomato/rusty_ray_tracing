use std::fmt::Display;

#[derive(Default, Debug, Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new() -> Self {
        Self(0, 0, 0)
    }

    pub fn red(&self) -> u8 {
        self.0
    }

    pub fn green(&self) -> u8 {
        self.1
    }

    pub fn blue(&self) -> u8 {
        self.2
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from(value: (f64, f64, f64)) -> Self {
        let rbyte = (255.999 * value.0) as u8;
        let gbyte = (255.999 * value.1) as u8;
        let bbyte = (255.999 * value.2) as u8;

        Self::from((rbyte, gbyte, bbyte))
    }
}
