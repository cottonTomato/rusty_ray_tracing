use crate::{
    rand::{random_f64, random_vec3_unit_hemisphere},
    Camera, Color, Hittable, Image, Interval, Ray, Vector3, INFINITY,
};

pub struct Renderer<'a> {
    camera: &'a Camera,
    image: &'a mut Image,
    sampling_per_px: u32,
    max_reflections: u32,
}

impl<'a> Renderer<'a> {
    pub fn render(&mut self, world: &impl Hittable) {
        let img_width = self.image.img_width();
        let img_height = self.image.img_height();

        for j in 0..img_height {
            eprintln!("\rScanlines Remaining: {} ", img_height - j);
            for i in 0..img_width {
                let mut color_vec = Vector3::new();

                for _ in 0..self.sampling_per_px {
                    let x = (i as f64) + random_f64() - 0.5;
                    let y = (j as f64) + random_f64() - 0.5;
                    let ray = self.camera.get_ray(x, y);
                    color_vec += Self::ray_color_vec(ray, world, self.max_reflections);
                }

                self.image
                    .set_px(i, j, Color::from(color_vec / self.sampling_per_px as f64));
            }
        }
    }

    fn ray_color_vec(mut ray: Ray, world: &impl Hittable, max_depth: u32) -> Vector3 {
        let mut color = Vector3::new();
        let mut multiplier = 1.0;

        for _ in 0..max_depth {
            if let Some(hit_record) = world.hit(ray, Interval::new(0.001, INFINITY)) {
                let new_direction = random_vec3_unit_hemisphere(hit_record.normal);
                ray = Ray::new(hit_record.point, new_direction);
                multiplier *= 0.5;
                continue;
            }

            let unit_directions = ray.direction().normalized();
            let a = 0.5 * (unit_directions.y + 1.0);
            color = ((Vector3::from((1.0, 1.0, 1.0)) * (1.0 - a))
                + (Vector3::from((0.5, 0.7, 1.0)) * a))
                * multiplier;
            break;
        }

        color
    }

    #[allow(dead_code)]
    fn ray_color_vec_recurse(ray: Ray, world: &impl Hittable, current_depth: u32) -> Vector3 {
        if current_depth == 0 {
            return Vector3::new();
        }

        if let Some(hit_record) = world.hit(ray, Interval::new(0.0, INFINITY)) {
            let new_direction = random_vec3_unit_hemisphere(hit_record.normal);
            return (Self::ray_color_vec_recurse(
                Ray::new(hit_record.point, new_direction),
                world,
                current_depth - 1,
            )) * 0.5;
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
    max_reflection: Option<u32>,
}

impl<'a> RendererBuilder<'a> {
    pub fn new() -> Self {
        Self {
            camera: None,
            image: None,
            sample_per_px: None,
            max_reflection: None,
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

    pub fn with_max_reflections(mut self, max_reflection: u32) -> Self {
        self.max_reflection = Some(max_reflection);
        self
    }

    pub fn build(self) -> Renderer<'a> {
        Renderer {
            camera: self.camera.unwrap(),
            image: self.image.unwrap(),
            sampling_per_px: self.sample_per_px.unwrap(),
            max_reflections: self.max_reflection.unwrap(),
        }
    }
}
