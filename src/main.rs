mod color;
mod ray;
mod vec3;

fn main() {
    render_image();
}

fn render_image() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let mut r;
    let mut g;
    let mut b;

    for h_idx in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - h_idx);
        for w_idx in 0..image_width {
            r = w_idx as f64 / ((image_width - 1) as f64);
            g = h_idx as f64 / ((image_height - 1) as f64);
            b = 0 as f64;

            let pixel_color: color::Color = color::Color::new(r, g, b);
            color::write_color(pixel_color);
        }
    }

    eprintln!("\ndone");
}
