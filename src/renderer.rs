use crate::{Camera, Color, Hittable, Interval, Ray, Vector3, INFINITY};

pub struct Renderer<'a> {
    camera: &'a Camera,
}

impl<'a> Renderer<'a> {
    pub fn new(camera: &'a Camera) -> Self {
        Self { camera }
    }

    pub fn render(&self, world: &impl Hittable) {
        let img_width = self.camera.img_attr().img_width;
        let img_height = self.camera.img_attr().img_height;
        let camera_center = self.camera.position();
        let pixel_du = self.camera.du();
        let pixel_dv = self.camera.dv();
        let pixel00 = self.camera.image_origin();

        println!("P3\n{img_width} {img_height}\n255");

        for j in 0..img_height {
            eprintln!("\rScanlines Remaining: {} ", img_height - j);
            for i in 0..img_width {
                let pixel_center = pixel00 + (pixel_du * i as f64) + (pixel_dv * j as f64);
                let ray_direction = pixel_center - camera_center;
                let ray = Ray::new(camera_center, ray_direction);
                let color = Self::ray_color(ray, world);

                println!("{color}");
            }
        }
    }

    fn ray_color(ray: Ray, world: &impl Hittable) -> Color {
        if let Some(hit_record) = world.hit(ray, Interval::new(0.0, INFINITY)) {
            return Color::from((hit_record.normal + Vector3::from_float(1.0, 1.0, 1.0)) * 0.5);
        }

        let unit_directions = ray.direction().normalized();
        let a = 0.5 * (unit_directions.y() + 1.0);
        let b = (Vector3::from((1.0, 1.0, 1.0)) * (1.0 - a)) + (Vector3::from((0.5, 0.7, 1.0)) * a);
        Color::from(b)
    }
}
