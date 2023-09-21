use auto_ops::impl_op_ex;
use rand::Rng;
use std::fmt;

const VEC3_EPS: f64 = 1e-8;

#[derive(Clone, Debug)]
pub struct Vec3 {
    // ...
    e: [f64; 3],
}

impl Vec3 {
    // ...
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn from(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        Vec3::from(rand::random(), rand::random(), rand::random())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut a: [f64; 3] = [0.0, 0.0, 0.0];
        for i in 0..3 {
            a[i] = rand::thread_rng().gen_range(min..max);
        }
        Vec3::from(a[0], a[1], a[2])
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn near_zero_in_all_dimensions(&self) -> bool {
        self.x() < VEC3_EPS && self.y() < VEC3_EPS && self.z() < VEC3_EPS
    }
    pub fn dot(&self, v: &Vec3) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * v[2] - self[2] * v[1],
                self[2] * v[0] - self[0] * v[2],
                self[0] * v[1] - self[1] * v[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().unit_vector()
    }

    // 输入反射法向量，返回一个与法向量有正方向分量的随机向量
    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot(&on_unit_sphere, self) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }
}

// v[1]
impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl_op_ex!(-|a: &Vec3| -> Vec3 { Vec3::from(-a.x(), -a.y(), -a.z()) });
impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::from(a.x() + b.x(), a.y() + b.y(), a.z() + b.z()) });
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { a + (-b) });
impl_op_ex!(*|a: &Vec3, b: &f64| -> Vec3 { Vec3::from(a.x() * b, a.y() * b, a.z() * b) });
impl_op_ex!(*|a: &f64, b: &Vec3| -> Vec3 { b * a });
// 就是点乘
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::from(a.x() * b.x(), a.y() * b.y(), a.z() * b.z())
});
impl_op_ex!(/|a: &Vec3, b: &f64| -> Vec3 { a * (1.0 / b) });

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        // 这为什么是合法的啊？
        *self = *self + rhs;
    }
}

impl Vec3 {
    pub fn reflect(&self, reflective_normal: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(reflective_normal) * reflective_normal
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
