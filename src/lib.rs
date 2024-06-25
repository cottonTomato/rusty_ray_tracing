mod color;
mod hittable;
pub mod hittables;
mod interval;
mod ray;
mod vec3;

pub use color::Color;
pub use hittable::{HitRecord, Hittable};
pub use interval::Interval;
pub use ray::Ray;
pub use vec3::{Point3D, Vector3};

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degree_to_rad(degree: f64) -> f64 {
    degree * PI / 180.0
}
