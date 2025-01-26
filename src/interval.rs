pub struct Interval {
    min: f32,
    max: f32,
}

/// EMPTY represents an interval with nothing.
pub const EMPTY: Interval = Interval {
    min: f32::INFINITY,
    max: f32::NEG_INFINITY,
};
/// UNIVERSE represents an interval with everything.
pub const UNIVERSE: Interval = Interval {
    min: f32::NEG_INFINITY,
    max: f32::INFINITY,
};

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        assert!(min <= max);
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn surrounds(&self, val: f32) -> bool {
        self.min < val && val < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }
}
