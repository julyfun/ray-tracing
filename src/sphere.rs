use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Vec3};
use std::rc::Rc;
use vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    // 返回射线是否碰撞，以及返回碰撞点和碰撞表面的出法向量
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let h = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (-h - sqrt_d) / a;
        // 寻找需求范围内的解
        if !ray_t.surrounds(root) {
            root = (-h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        // is a unit vector
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::from(&p, t, r, &outward_normal))
    }
}
