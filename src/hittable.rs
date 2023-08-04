use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}
