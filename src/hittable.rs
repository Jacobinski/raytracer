use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub pt: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
}

impl HitRecord {
    /// Sets the hit `normal` vector and updates `is_front_face` to be the correct
    /// value for the given ray. This method should be called during geometry
    /// computation.
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        assert!(f32::abs(outward_normal.length() - 1.0) < 0.01);
        self.is_front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    /// Determines if a ray intersects the object within the given time range.
    fn hit(&self, r: Ray, time: &Interval) -> Option<HitRecord>;
}
