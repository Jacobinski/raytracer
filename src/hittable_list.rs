use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
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

    pub fn hit(&self, r: Ray, time: &Interval) -> Option<HitRecord> {
        let mut closest: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = object.hit(r, time) {
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
