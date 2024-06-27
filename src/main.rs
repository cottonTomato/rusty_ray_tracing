use ray_tracing::{
    geometries, image_writers::PpmWriter, Camera, HittableList, Image, RendererBuilder, Vector3,
};
use std::rc::Rc;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 1600;
    let mut image = Image::new(img_width, aspect_ratio);

    // Camera
    let camera = Camera::new(image.img_width(), image.img_height());

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(geometries::Sphere::new(
        Vector3::from_floats(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(geometries::Sphere::new(
        Vector3::from_floats(0.0, -100.5, -1.0),
        100.0,
    )));

    // Render
    let mut renderer = RendererBuilder::new()
        .using(&camera)
        .on(&mut image)
        .with_super_sampliing(10)
        .build();
    eprintln!("Rendering Image...");
    renderer.render(&world);

    // Writing Image to File
    eprintln!("Writing Image...");
    image
        .write("./out/img.ppm", PpmWriter)
        .expect("Unable to Write to file");
}
