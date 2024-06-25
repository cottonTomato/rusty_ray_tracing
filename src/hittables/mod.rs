use crate::{HitRecord, Hittable, Interval, Ray};
use std::rc::Rc;

mod sphere;

pub use sphere::Sphere;

#[derive(Default)]
pub struct HitList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HitList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn new_populated(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut closest_record = None;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_record.t;
                closest_record = Some(hit_record);
            }
        }

        closest_record
    }
}
