use crate::{
    rand::{random_f64, random_vec3_unit_hemisphere},
    Camera, Color, Hittable, Image, Interval, Ray, Vector3, INFINITY,
};

pub struct Renderer<'a> {
    camera: &'a Camera,
    image: &'a mut Image,
    sampling_per_px: u32,
}

impl<'a> Renderer<'a> {
    pub fn render(&mut self, world: &impl Hittable) {
        let img_width = self.image.img_width();
        let img_height = self.image.img_height();

        for j in 0..img_height {
            eprintln!("\rScanlines Remaining: {} ", img_height - j);
            for i in 0..img_width {
                // Old Method without Anti-Alising

                // let color = Self::ray_color(self.camera.get_ray(i as f64, j as f64), world);
                // self.image.set_px(i, j, color);

                // New Method
                let mut color_vec = Vector3::new();
                for _ in 0..self.sampling_per_px {
                    let x = (i as f64) + random_f64() - 0.5;
                    let y = (j as f64) + random_f64() - 0.5;
                    let ray = self.camera.get_ray(x, y);
                    color_vec += Self::ray_color_vec(ray, world);
                }
                self.image
                    .set_px(i, j, Color::from(color_vec / self.sampling_per_px as f64));
            }
        }
    }

    // Old Method

    // fn ray_color(ray: Ray, world: &impl Hittable) -> Color {
    //     if let Some(hit_record) = world.hit(ray, Interval::new(0.0, INFINITY)) {
    //         return Color::from((hit_record.normal + Vector3::from_floats(1.0, 1.0, 1.0)) * 0.5);
    //     }

    //     let unit_directions = ray.direction().normalized();
    //     let a = 0.5 * (unit_directions.y + 1.0);
    //     let b = (Vector3::from((1.0, 1.0, 1.0)) * (1.0 - a)) + (Vector3::from((0.5, 0.7, 1.0)) * a);
    //     Color::from(b)
    // }

    fn ray_color_vec(ray: Ray, world: &impl Hittable) -> Vector3 {
        if let Some(hit_record) = world.hit(ray, Interval::new(0.0, INFINITY)) {
            return (hit_record.normal + Vector3::from_floats(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_directions = ray.direction().normalized();
        let a = 0.5 * (unit_directions.y + 1.0);
        (Vector3::from((1.0, 1.0, 1.0)) * (1.0 - a)) + (Vector3::from((0.5, 0.7, 1.0)) * a)
    }
}

#[derive(Default)]
pub struct RendererBuilder<'a> {
    camera: Option<&'a Camera>,
    image: Option<&'a mut Image>,
    sample_per_px: Option<u32>,
}

impl<'a> RendererBuilder<'a> {
    pub fn new() -> Self {
        Self {
            camera: None,
            image: None,
            sample_per_px: None,
        }
    }

    pub fn using(mut self, camera: &'a Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn on(mut self, image: &'a mut Image) -> Self {
        self.image = Some(image);
        self
    }

    pub fn with_super_sampliing(mut self, sampling_per_px: u32) -> Self {
        self.sample_per_px = Some(sampling_per_px);
        self
    }

    pub fn build(self) -> Renderer<'a> {
        Renderer {
            camera: self.camera.unwrap(),
            image: self.image.unwrap(),
            sampling_per_px: self.sample_per_px.unwrap(),
        }
    }
}
