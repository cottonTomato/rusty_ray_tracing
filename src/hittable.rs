use crate::{Interval, Point3D, Ray, Vector3};
use std::rc::Rc;

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    pub point: Point3D,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, ray: Ray, outward_normal_logic: impl Fn(Point3D) -> Vector3) -> Self {
        let point = ray.at(t);
        let outward_normal = outward_normal_logic(point);
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            point,
            normal,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
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

impl Hittable for HittableList {
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
