use crate::hittable;
use crate::ray;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }

    pub fn add(&mut self, object: Rc<dyn hittable::Hittable>) {
        self.objects.push(object);
    }
}

impl hittable::Hittable for HittableList {
    fn hit(
        &self,
        r: &ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let mut temp_rec: hittable::HitRecord = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone()
            }
        }

        hit_anything
    }
}
