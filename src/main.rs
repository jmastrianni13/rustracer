mod vec3;

fn main() {
    let vec3 = vec3::Vec3::new(1.0, 2.0, 3.0);
    println!("{}", vec3);
}

fn render_image() {
    let image_width = 256;
    let image_height = 256;
    let mplier = 255.999;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let mut r: f64;
    let mut g: f64;
    let b: f64 = 0.0;

    let mut ir: i32;
    let mut ig: i32;
    let mut ib: i32;

    for h_idx in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - h_idx);
        for w_idx in 0..image_width {
            r = w_idx as f64 / ((image_width - 1) as f64);
            g = h_idx as f64 / ((image_height - 1) as f64);

            ir = (mplier * r) as i32;
            ig = (mplier * g) as i32;
            ib = (mplier * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\ndone");
}
