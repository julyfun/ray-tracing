use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3;
use vec3::{Color, Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    // 返回射线是否碰撞，以及返回碰撞点和碰撞表面的出法向量
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let h = vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return (false, HitRecord::new());
        }
        let sqrt_d = discriminant.sqrt();
        let mut root = (-h - sqrt_d) / a;
        if !(t_min <= root && root <= t_max) {
            root = (-h + sqrt_d) / a;
            if !(t_min <= root && root <= t_max) {
                return (false, HitRecord::new());
            }
        }
        let t = root;
        let p = r.at(t);
        // is a unit vector
        let outward_normal = (p - self.center) / self.radius;
        (true, HitRecord::from(&p, t, r, &outward_normal))
    }
}
