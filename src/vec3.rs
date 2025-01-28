use core::f64;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::random::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn new_random_unit_vector(rng: &mut Rng) -> Self {
        let pi = f64::consts::PI;
        let phi = pi * rng.generate();
        let theta = 2.0 * pi * rng.generate();

        let x = f64::sin(phi) * f64::cos(theta);
        let y = f64::sin(phi) * f64::sin(theta);
        let z = f64::cos(phi);
        assert!(f64::abs(x * x + y * y + z * z - 1.0) <= 0.001);

        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]],
        }
    }
}

impl Mul<Vec3> for u32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self as f64 * rhs.e[0],
                self as f64 * rhs.e[1],
                self as f64 * rhs.e[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
