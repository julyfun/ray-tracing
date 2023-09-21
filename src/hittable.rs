use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

pub struct HitRecord {
    /// 碰撞点
    pub p: Point3,
    /// 碰撞平面反射方向的单位法向量
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    /// 碰撞方程的解
    pub t: f64,
    /// 碰撞是否由外向内
    pub front_face: bool,
}

impl HitRecord {
    // p: 碰撞点
    // t: 不知道由什么用，但是是入射光线的长度解
    // r: 入射光线
    pub fn from(p: &Point3, t: f64, r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
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
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
