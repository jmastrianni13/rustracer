use std::fmt;
use std::io::Write;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn unit_vector(&self) -> Self {
        return self.clone() / self.length();
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds for Vec3"),
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Vec3 {{ x:{}, y:{}, z:{} }}", self.x, self.y, self.z);
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        return Self::new(-self.x, -self.y, -self.z);
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        return self * (1 as f64 / t);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        return Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        return Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl AddAssign for Vec3 {
    // type is implied to be Vec3
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1 as f64 / t;
    }
}

impl MulAssign<f64> for Vec3 {
    // have to explicitly type f64 because it is not the same as self
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

fn get_hadamard_prod(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    return Vec3::new(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z);
}

fn get_dot_prod(lhs: &Vec3, rhs: &Vec3) -> f64 {
    return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z;
}

fn get_cross_prod(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    return Vec3::new(
        lhs.y * rhs.z - lhs.z * rhs.y,
        lhs.z * rhs.x - lhs.x * rhs.z,
        lhs.x * rhs.y - lhs.y * rhs.x,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(x: f64, y: f64) -> bool {
        // deal with floating point precision issues
        return (x - y).abs() < EPSILON;
    }

    #[test]
    fn build_vec3() {
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 2.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn get_negated_vec3() {
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        let neg_v3 = -v3;
        assert_eq!(neg_v3.x, -1.0);
        assert_eq!(neg_v3.y, -2.0);
        assert_eq!(neg_v3.z, -3.0);
    }

    #[test]
    fn mut_add_vec3() {
        let mut v3 = Vec3::new(1.0, 2.0, 3.0);
        v3 += Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(v3.x, 5.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 7.0);
    }

    #[test]
    fn mut_mul_vec3() {
        let mut v3 = Vec3::new(1.0, 2.0, 3.0);
        v3 *= 3.0;
        assert_eq!(v3.x, 3.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 9.0);
    }

    #[test]
    fn mut_div_vec3() {
        let mut v3 = Vec3::new(2.0, 4.0, 6.0);
        v3 /= 2.0;
        assert_eq!(v3.x, 1.0);
        assert_eq!(v3.y, 2.0);
        assert_eq!(v3.z, 3.0);
    }

    #[test]
    fn index_vec3() {
        let mut v3 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v3[0], 1.0);
        assert_eq!(v3[1], 2.0);
        assert_eq!(v3[2], 3.0);
    }

    #[test]
    fn vec3_length_squared() {
        let mut v3 = Vec3::new(0.0, 3.0, 4.0);
        assert_eq!(v3.length_squared(), 25.0);
    }

    #[test]
    fn vec3_length() {
        let mut v3 = Vec3::new(0.0, 3.0, 4.0);
        assert_eq!(v3.length(), 5.0);
    }

    #[test]
    fn vec3_unit_vector() {
        let v3 = Vec3::new(0.0, 3.0, 4.0);
        let v3_unit = v3.unit_vector();
        assert_eq!(v3_unit.x, 0.0);
        assert!(approx_eq(v3_unit.y, 0.6));
        assert_eq!(v3_unit.z, 0.8);
        assert_eq!(v3.x, 0.0);
        assert_eq!(v3.y, 3.0);
        assert_eq!(v3.z, 4.0);
    }

    #[test]
    fn vec3_add() {
        let v3_a = Vec3::new(1.0, 2.0, 3.0);
        let v3_b = Vec3::new(4.0, 5.0, 6.0);
        let v3_c = v3_a + v3_b;
        assert_eq!(v3_c.x, 5.0);
        assert_eq!(v3_c.y, 7.0);
        assert_eq!(v3_c.z, 9.0);
    }

    #[test]
    fn vec3_mul() {
        let v3 = Vec3::new(1.0, 2.0, 3.0);
        let v3_3 = v3 * 3.0;
        assert_eq!(v3_3.x, 3.0);
        assert_eq!(v3_3.y, 6.0);
        assert_eq!(v3_3.z, 9.0);
    }

    #[test]
    fn vec3_div() {
        let v3 = Vec3::new(3.0, 6.0, 9.0);
        let v3_3 = v3 / 3.0;
        assert_eq!(v3_3.x, 1.0);
        assert_eq!(v3_3.y, 2.0);
        assert_eq!(v3_3.z, 3.0);
    }

    #[test]
    fn vec3_sub() {
        let v3_a = Vec3::new(1.0, 2.0, 3.0);
        let v3_b = Vec3::new(4.0, 5.0, 6.0);
        let v3_c = v3_a - v3_b;
        assert_eq!(v3_c.x, -3.0);
        assert_eq!(v3_c.y, -3.0);
        assert_eq!(v3_c.z, -3.0);
    }

    #[test]
    fn vec3_hadamard_prod() {
        let v3_a = Vec3::new(1.0, 2.0, 3.0);
        let v3_b = Vec3::new(4.0, 5.0, 6.0);
        let v3_c = get_hadamard_prod(&v3_a, &v3_b);
        assert_eq!(v3_c.x, 4.0);
        assert_eq!(v3_c.y, 10.0);
        assert_eq!(v3_c.z, 18.0);
        assert_eq!(v3_a.x, 1.0);
        assert_eq!(v3_a.y, 2.0);
        assert_eq!(v3_a.z, 3.0);
        assert_eq!(v3_b.x, 4.0);
        assert_eq!(v3_b.y, 5.0);
        assert_eq!(v3_b.z, 6.0);
    }

    #[test]
    fn vec3_dot_prod() {
        let v3_a = Vec3::new(1.0, 2.0, 3.0);
        let v3_b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(get_dot_prod(&v3_a, &v3_b), 32.0);
        assert_eq!(v3_a.x, 1.0);
        assert_eq!(v3_a.y, 2.0);
        assert_eq!(v3_a.z, 3.0);
        assert_eq!(v3_b.x, 4.0);
        assert_eq!(v3_b.y, 5.0);
        assert_eq!(v3_b.z, 6.0);
    }

    #[test]
    fn vec3_cross_prod() {
        let v3_a = Vec3::new(1.0, 2.0, 3.0);
        let v3_b = Vec3::new(4.0, 5.0, 6.0);
        let v3_c = get_cross_prod(&v3_a, &v3_b);
        assert_eq!(v3_c.x, -3.0); // 2 * 6 - 3 * 5
        assert_eq!(v3_c.y, 6.0); // 3 * 4 - 1 * 6
        assert_eq!(v3_c.z, -3.0); // 1 * 5 - 2 * 4
        assert_eq!(v3_a.x, 1.0);
        assert_eq!(v3_a.y, 2.0);
        assert_eq!(v3_a.z, 3.0);
        assert_eq!(v3_b.x, 4.0);
        assert_eq!(v3_b.y, 5.0);
        assert_eq!(v3_b.z, 6.0);
    }
}
