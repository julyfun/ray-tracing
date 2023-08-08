use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    // 碰撞是否由外向内
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        }
    }

    // p: 碰撞点
    // t: 不知道由什么用，但是是入射光线的长度解
    // r: 入射光线
    pub fn from(p: &Point3, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = vec3::dot(&r.direction(), outward_normal) < 0.0;
        // 如果由外向内，则法向量方向向外
        let normal = if front_face {
            outward_normal.clone()
        } else {
            // 否则法向量方向向内
            -outward_normal.clone()
        };
        Self {
            p: p.clone(),
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}
