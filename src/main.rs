use std::io::Write;

fn out_gradient() {
    let img_width = 256;
    let img_height = 256;

    println!("P3\n{img_width} {img_height}\n255");

    for i in 0..img_height {
        eprintln!("\rScanlines Remaining: {} ", img_height - i);
        std::io::stderr().flush().unwrap();

        for j in 0..img_width {
            let r = j as f64 / (img_height - 1) as f64;
            let g = i as f64 / (img_width - 1) as f64;
            let b = 0.0;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            println!("{ir} {ig} {ib}");
        }
    }
}

fn main() {
    out_gradient();
}
