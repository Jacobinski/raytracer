use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        assert!(radius >= 0.0);
        Self { radius, center }
    }
}

impl Hittable for Sphere {
    fn hit(self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (h - f32::sqrt(discriminant)) / a;
        if root <= t_min || root >= t_max {
            root = (h + f32::sqrt(discriminant)) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        let time = root;
        let collision_point = r.at(time);
        let collision_normal = (collision_point - self.center) / self.radius;
        Some(HitRecord {
            t: time,
            pt: collision_point,
            normal: collision_normal,
        })
    }
}
