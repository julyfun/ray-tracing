use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut record = HitRecord::new();
        let mut closest = ray_t.max;
        for obj in self.objects.iter() {
            let (hit, rec) = obj.hit(r, Interval::from(ray_t.min, closest));
            if hit {
                hit_anything = true;
                closest = rec.t;
                record = rec;
            }
        }
        return (hit_anything, record);
    }
}
