use std::io::{self, Write};
use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

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
struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self(x, y, z);
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        return Vec3::new(-self.0, -self.1, -self.2);
    }
}

impl AddAssign for Vec3 {
    // type is implied to be Vec3
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl MulAssign<f64> for Vec3 {
    // have to explicitly type f64 because it is not the same as self
    fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1 as f64 / t;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_vec3() {
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v3.0, 1.0);
        assert_eq!(v3.1, 2.0);
        assert_eq!(v3.2, 3.0);
    }

    #[test]
    fn get_negated_vec3() {
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        let neg_v3 = -v3;
        assert_eq!(neg_v3.0, -1.0);
        assert_eq!(neg_v3.1, -2.0);
        assert_eq!(neg_v3.2, -3.0);
    }

    #[test]
    fn add_to_vec3() {
        let mut v3 = Vec3::new(1.0, 2.0, 3.0);
        v3 += Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(v3.0, 5.0);
        assert_eq!(v3.1, 6.0);
        assert_eq!(v3.2, 7.0);
    }

    #[test]
    fn mult_vec3() {
        let mut v3 = Vec3::new(1.0, 2.0, 3.0);
        v3 *= 3.0;
        assert_eq!(v3.0, 3.0);
        assert_eq!(v3.1, 6.0);
        assert_eq!(v3.2, 9.0);
    }

    #[test]
    fn div_vec3() {
        let mut v3 = Vec3::new(2.0, 4.0, 6.0);
        v3 /= 2.0;
        assert_eq!(v3.0, 1.0);
        assert_eq!(v3.1, 2.0);
        assert_eq!(v3.2, 3.0);
    }
}
