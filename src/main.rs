use ray_tracing::{Color, Point3D, Ray, Vector3};

fn hit_sphere(center: Point3D, radius: f64, ray: Ray) -> f64 {
    let oc = center - ray.origin();
    let a = ray.direction().length_squared();
    let h = ray.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h + discriminant.sqrt()) / a
    }
}

fn ray_color(ray: Ray) -> Color {
    let center_of_sphere = Point3D::from_float(0.0, 0.0, -1.0);
    let t = hit_sphere(center_of_sphere, 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - center_of_sphere).normalized();
        return Color::from(Vector3::from_float(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5);
    }

    let unit_directions = ray.direction().normalized();
    let a = 0.5 * (unit_directions.y() + 1.0);
    let b = (Vector3::from((1.0, 1.0, 1.0)) * (1.0 - a)) + (Vector3::from((0.5, 0.7, 1.0)) * a);
    Color::from(b)
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
