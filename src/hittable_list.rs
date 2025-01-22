use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::default(),
        }
    }

    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.objects.push(item);
    }

    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = object.hit(r, t_min, t_max) {
                match closest {
                    Some(c) => {
                        if record.t < c.t {
                            closest = Some(record)
                        }
                    }
                    None => closest = Some(record),
                }
            }
        }
        closest
    }
}
