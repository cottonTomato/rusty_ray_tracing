use ray_tracing::{Color, Point3D, Ray, Vector3};

fn ray_color(_ray: &Ray) -> Color {
    Color::new()
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 1600;
    let img_height = (img_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64 / img_height as f64);
    let focal_length = 1.0;
    let camera_center = Point3D::new();

    let viewport_u = Point3D::from_float(viewport_width, 0.0, 0.0);
    let viewport_v = Point3D::from_float(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / img_width as f64;
    let pixel_delta_v = viewport_v / img_height as f64;

    let viewport_upper_left = camera_center
        - Vector3::from_float(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{img_width} {img_height}\n255");

    for j in 0..img_height {
        eprintln!("\rScanlines Remaining: {} ", img_height - j);
        for i in 0..img_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(ray);

            println!("{color}");
        }
    }
}
