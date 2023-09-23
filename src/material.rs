use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

pub struct Scattered {
    /// 翻译：衰减量
    pub attenuation: Color,
    pub ray: Ray,
}

pub trait Material {
    /// 可以决定光线的反射后的方向和颜色
    /// 材质 => 物体 => 可以被相机光线碰撞 => 碰撞时材质对碰撞参数进行反射
    fn scatter(
        &self,
        r_in: &Ray,
        hit_pos: Point3,
        hit_face_normal: Vec3,
        hit_front_face: bool,
    ) -> Option<Scattered>;
}

pub struct Lambertian {
    /// 对各个颜色的反射率
    albedo: Color,
}

impl Lambertian {
    pub fn from(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        #[allow(unused)] r_in: &Ray,
        hit_pos: Point3,
        hit_face_normal: Vec3,
        hit_front_face: bool,
    ) -> Option<Scattered> {
        let scatter_direction = hit_face_normal + Vec3::random_unit_vector();
        let scatter_direction = if scatter_direction.near_zero_in_all_dimensions() {
            hit_face_normal
        } else {
            scatter_direction
        };
        let scattered_ray = Ray::from(hit_pos, scatter_direction);
        Some(Scattered {
            attenuation: self.albedo,
            ray: scattered_ray,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn from(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_pos: Point3,
        hit_face_normal: Vec3,
        hit_front_face: bool,
    ) -> Option<Scattered> {
        let reflected_direction = r_in.direction().unit_vector().reflect(hit_face_normal);
        Some(Scattered {
            attenuation: self.albedo,
            ray: Ray::from(hit_pos, reflected_direction),
        })
    }
}

pub struct FuzzyMetal {
    albedo: Color,
    fuzz: f64,
}

impl FuzzyMetal {
    pub fn from(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for FuzzyMetal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_pos: Point3,
        hit_face_normal: Vec3,
        hit_front_face: bool,
    ) -> Option<Scattered> {
        let reflected_direction = r_in.direction().unit_vector().reflect(hit_face_normal);
        let fuzzy_direction = reflected_direction + self.fuzz * Vec3::random_unit_vector();
        if fuzzy_direction.dot(hit_face_normal) > 0.0 {
            Some(Scattered {
                attenuation: self.albedo,
                ray: Ray::from(hit_pos, fuzzy_direction),
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn from(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_pos: Point3,
        hit_face_normal: Vec3,
        hit_front_face: bool,
    ) -> Option<Scattered> {
        let refraction_ratio = if hit_front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let refracted = r_in
            .direction()
            .unit_vector()
            .refract(hit_face_normal, refraction_ratio);
        let scattered = Scattered {
            attenuation: Color::from(1.0, 1.0, 1.0),
            ray: Ray::from(hit_pos, refracted),
        };
        Some(scattered)
    }
}
