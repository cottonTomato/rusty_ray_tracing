use crate::{HitRecord, Hittable, Point3D, Ray};

pub struct Sphere {
    center: Point3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;
        if root <= t_min || root >= t_max {
            root = (h + sqrt_discriminant) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }

        Some(HitRecord::new(root, ray, |p| {
            (p - self.center) / self.radius
        }))
    }
}
