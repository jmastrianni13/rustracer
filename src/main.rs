mod color;
mod ray;
mod vec3;

fn main() {
    render_image();
}

fn hit_sphere(center: ray::Point3, radius: f64, r: &ray::Ray) -> f64 {
    let oc = center - r.orig.clone();
    let a = r.dir.length_squared();
    let h = vec3::get_dot_prod(&r.dir, &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - f64::sqrt(discriminant)) / a;
    }
}

fn ray_color(r: &ray::Ray) -> color::Color {
    let t = hit_sphere(ray::Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - vec3::Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return color::Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_direction = r.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    return color::Color::new(1.0, 1.0, 1.0) * (1.0 - a) + color::Color::new(0.5, 0.7, 1.0) * a;
}

fn render_image() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).max(1.0) as i32;

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = ray::Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u.clone() / image_width as f64;
    let pixel_delta_v = viewport_v.clone() / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center.clone()
        - vec3::Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc =
        viewport_upper_left.clone() + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let mut pixel_center;
    let mut ray_direction;
    let mut r: ray::Ray;
    let mut pixel_color;

    for h_idx in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - h_idx);
        for w_idx in 0..image_width {
            pixel_center = pixel00_loc.clone()
                + (pixel_delta_u.clone() * w_idx as f64)
                + (pixel_delta_v.clone() * h_idx as f64);
            ray_direction = pixel_center - camera_center.clone();
            r = ray::Ray::new(camera_center.clone(), ray_direction);
            pixel_color = ray_color(&r);
            color::write_color(pixel_color);
        }
    }

    eprintln!("\ndone");
}
