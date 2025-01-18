use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
