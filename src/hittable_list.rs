use crate::hittable;
use crate::interval;
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
    fn hit(&self, r: &ray::Ray, ray_t: interval::Interval, rec: &mut hittable::HitRecord) -> bool {
        let mut temp_rec: hittable::HitRecord = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let ray_int = interval::Interval {
                min: ray_t.min,
                max: closest_so_far,
            };
            if object.hit(r, ray_int, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone()
            }
        }

        hit_anything
    }
}
