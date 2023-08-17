use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;

struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut record = HitRecord::new();
        let mut closest = t_max;
        for obj in self.objects.iter() {
            let (hit, rec) = obj.hit(r, t_min, closest);
            if hit {
                hit_anything = true;
                closest = rec.t;
                record = rec;
            }
        }
        return (hit_anything, record);
    }
}
