use crate::ray;
use crate::vec3;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: ray::Point3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: ray::Point3, normal: vec3::Vec3, t: f64, front_face: bool) -> Self {
        return HitRecord {
            p,
            normal,
            t,
            front_face,
        };
    }

    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &vec3::Vec3) {
        self.front_face = vec3::get_dot_prod(&r.dir, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
