use crate::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(max: f64, min: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub const UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };
    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: -INFINITY,
            max: INFINITY,
        }
    }
}
