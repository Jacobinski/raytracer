/// RNG is a random number generator using the Linear Congruential Generator algorithm.
pub struct Rng {
    x: u64,
}

// Parameters from Knuth and H. W. Lewis.
// https://en.wikipedia.org/wiki/Linear_congruential_generator
const A: u64 = 1664525;
const C: u64 = 1013904223;
const M: u64 = 1 << 32;

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng { x: seed }
    }

    // Generates a random number in the range [0, 1)
    pub fn generate(&mut self) -> f64 {
        self.x = (A * self.x + C) % M;
        self.x as f64 / M as f64
    }
}
