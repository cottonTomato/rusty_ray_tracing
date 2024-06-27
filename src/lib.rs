mod camera;
pub use camera::Camera;

mod color;
pub use color::Color;

mod hittable;
pub use hittable::{HitRecord, Hittable, HittableList};
pub mod geometries;

mod image;
pub use image::{Image, ImgWriter};
pub mod image_writers;

mod interval;
use interval::Interval;

mod ray;
use ray::Ray;

pub mod rand;

mod renderer;
pub use renderer::{Renderer, RendererBuilder};

mod vec3;
pub use vec3::{Point3D, Vector3};

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degree_to_rad(degree: f64) -> f64 {
    degree * PI / 180.0
}
