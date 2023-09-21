use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

struct Scattered {
    /// 翻译：衰减量
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    /// 可以决定光线的反射后的方向和颜色
    /// 材质 => 物体 => 可以被相机光线碰撞 => 碰撞时材质对碰撞参数进行反射
    fn scatter(&self, r_in: &Ray, hit_pos: &Point3, hit_face_normal: &Vec3) -> Option<Scattered>;
}

struct Lambertian {
    /// 对各个颜色的反射率
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_pos: &Point3, hit_face_normal: &Vec3) -> Option<Scattered> {
        let scatter_direction = hit_face_normal + Vec3::random_unit_vector();
        let scatter_direction = if scatter_direction.near_zero_in_all_dimensions() {
            hit_face_normal.clone()
        } else {
            scatter_direction
        };
        let scattered_ray = Ray::from(hit_pos.clone(), scatter_direction);
        Some(Scattered {
            attenuation: self.albedo,
            ray: scattered_ray,
        })
    }
}
