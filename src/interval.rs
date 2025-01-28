pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        assert!(min <= max);
        Self { min, max }
    }

    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}
