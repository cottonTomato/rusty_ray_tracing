use ray_tracing::{hittables, Camera, Renderer, Vector3};
use std::rc::Rc;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 1600;

    // Camera
    let camera = Camera::new(img_width, aspect_ratio);

    // World
    let mut world = hittables::HitList::new();
    world.add(Rc::new(hittables::Sphere::new(
        Vector3::from_float(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(hittables::Sphere::new(
        Vector3::from_float(0.0, -100.5, -1.0),
        100.0,
    )));

    // Render
    Renderer::new(&camera).render(&world);
}
