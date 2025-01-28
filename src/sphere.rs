use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        assert!(radius >= 0.0);
        Self { radius, center }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, time: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (h - f64::sqrt(discriminant)) / a;
        if !time.surrounds(root) {
            root = (h + f64::sqrt(discriminant)) / a;
            if !time.surrounds(root) {
                return None;
            }
        }

        let time = root;
        let collision_point = r.at(time);
        let outward_normal = (collision_point - self.center) / self.radius;
        let mut record = HitRecord {
            t: time,
            pt: collision_point,
            normal: outward_normal,
            is_front_face: false,
        };
        record.set_face_normal(r, outward_normal);
        Some(record)
    }
}
