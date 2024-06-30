use std::io::{self, Write};
use std::ops::{AddAssign, DivAssign, MulAssign};

fn main() {
    let vec3 = Vec3::new(0.0, 0.0, 0.0);
    println!("{:?}", vec3);
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
        //io::stderr().flush().unwrap();
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

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Vec3 {
    point: Point,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let mut point = Point { x, y, z };
        return Self { point };
    }

    pub fn negate(self) -> Point {
        return Point {
            x: -self.point.x,
            y: -self.point.y,
            z: -self.point.z,
        };
    }

    pub fn x(self) -> f64 {
        return self.point.x;
    }

    pub fn y(self) -> f64 {
        return self.point.y;
    }

    pub fn z(self) -> f64 {
        return self.point.z;
    }
}

impl AddAssign for Vec3 {
    // type is implied to be Vec3
    fn add_assign(&mut self, rhs: Vec3) {
        self.point.x += rhs.point.x;
        self.point.y += rhs.point.y;
        self.point.z += rhs.point.z;
    }
}

impl MulAssign<f64> for Vec3 {
    // have to explicitly type f64 because it is not the same as self
    fn mul_assign(&mut self, t: f64) {
        self.point.x *= t;
        self.point.y *= t;
        self.point.z *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1 as f64 / t;
    }
}
