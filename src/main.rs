use ray_tracing::Color;

fn main() {
    let img_width = 256;
    let img_height = 256;

    println!("P3\n{img_width} {img_height}\n255");

    for i in 0..img_height {
        eprintln!("\rScanlines Remaining: {} ", img_height - i);

        for j in 0..img_width {
            let r = j as f64 / (img_height - 1) as f64;
            let g = i as f64 / (img_width - 1) as f64;
            let b = 0.0;

            let color = Color::from((r, g, b));

            println!("{}", color);
        }
    }
}
