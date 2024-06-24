use crate::{Point3D, Ray, Vector3};

pub mod sphere;

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
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
