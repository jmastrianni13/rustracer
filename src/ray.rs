use crate::vec3;

pub type Point3 = vec3::Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: vec3::Vec3) -> Self {
        return Self { orig, dir };
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.orig.clone() + self.dir.clone() * t;
    }

    fn direction(&self) -> &vec3::Vec3 {
        return &self.dir;
    }

    fn origin(&self) -> &Point3 {
        return &self.orig;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ray() {
        let orig_v = Point3::new(0.0, 0.0, 0.0);
        let dir_v = vec3::Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(orig_v, dir_v);
        assert_eq!(r.orig.x, 0.0);
        assert_eq!(r.orig.y, 0.0);
        assert_eq!(r.orig.z, 0.0);
        assert_eq!(r.dir.x, 1.0);
        assert_eq!(r.dir.y, 2.0);
        assert_eq!(r.dir.z, 3.0);
    }

    #[test]
    fn ray_direction() {
        let orig_v = Point3::new(0.0, 0.0, 0.0);
        let dir_v = vec3::Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(orig_v, dir_v);
        assert_eq!(r.dir.x, 1.0);
        assert_eq!(r.dir.y, 2.0);
        assert_eq!(r.dir.z, 3.0);

        let r_dir = r.direction();
        assert_eq!(r_dir.x, 1.0);
        assert_eq!(r_dir.y, 2.0);
        assert_eq!(r_dir.z, 3.0);
    }

    #[test]
    fn ray_origin() {
        let orig_v = Point3::new(0.0, 0.0, 0.0);
        let dir_v = vec3::Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(orig_v, dir_v);
        assert_eq!(r.orig.x, 0.0);
        assert_eq!(r.orig.y, 0.0);
        assert_eq!(r.orig.z, 0.0);

        let r_orig = r.origin();
        assert_eq!(r_orig.x, 0.0);
        assert_eq!(r_orig.y, 0.0);
        assert_eq!(r_orig.z, 0.0);
    }

    #[test]
    fn ray_point3() {
        let orig_v = Point3::new(1.0, 1.0, 1.0);
        let dir_v = vec3::Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(orig_v, dir_v);

        let r_point3 = r.at(3.0);
        assert_eq!(r_point3.x, 4.0);
        assert_eq!(r_point3.y, 7.0);
        assert_eq!(r_point3.z, 10.0);
    }
}
