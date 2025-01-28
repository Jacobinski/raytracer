pub struct Interval {
    min: f64,
    max: f64,
}

/// EMPTY represents an interval with nothing.
pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};
/// UNIVERSE represents an interval with everything.
pub const UNIVERSE: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        assert!(min <= max);
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
