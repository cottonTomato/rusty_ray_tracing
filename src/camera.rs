use crate::{ray::Ray, Point3D, Vector3};

pub struct Camera {
    position: Point3D,
    pixel00_loc: Point3D,
    pixel_du: Vector3,
    pixel_dv: Vector3,
}

impl Camera {
    pub fn new(img_width: u32, img_height: u32) -> Self {
        let center = Point3D::from_floats(0.0, 0.0, 0.0);

        // Viewport Dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (img_width as f64 / img_height as f64);

        let viewport_u = Vector3::from_floats(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::from_floats(0.0, -viewport_height, 0.0);

        let pixel_du = viewport_u / img_width as f64;
        let pixel_dv = viewport_v / img_height as f64;

        let viewport_origin = center
            - Vector3::from_floats(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let pixel00_loc = viewport_origin + (pixel_du + pixel_dv) * 0.5;

        Self {
            position: center,
            pixel00_loc,
            pixel_du,
            pixel_dv,
        }
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        let pixel_center = self.pixel00_loc + self.pixel_du * i + self.pixel_dv * j;
        let ray_direction = pixel_center - self.position;
        Ray::new(self.position, ray_direction)
    }
}
