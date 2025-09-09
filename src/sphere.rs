use crate::hittable;
use crate::ray;
use crate::vec3;

pub struct Sphere {
    center: ray::Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: ray::Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        r: &ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let oc = self.center.clone() - r.orig.clone();
        let a = r.dir.length_squared();
        let h = vec3::get_dot_prod(&r.dir, &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);
        let root = (h - sqrtd) / a;

        if (root <= ray_tmin) || (ray_tmax <= root) {
            return false;
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p.clone() - self.center.clone()) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}
