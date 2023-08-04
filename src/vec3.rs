use auto_ops::impl_op;
use auto_ops::impl_op_ex;
use std::fmt;

#[derive(Copy, Clone, Debug)]
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

    pub fn x(self) -> f64 {
        self.e[0]
    }
    pub fn y(self) -> f64 {
        self.e[1]
    }
    pub fn z(self) -> f64 {
        self.e[2]
    }
    pub fn length_squared(self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

// v[1]
impl std::ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl_op!(-|a: &Vec3| -> Vec3 { Vec3::from(-a.x(), -a.y(), -a.z()) });
impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::from(a.x() + b.x(), a.y() + b.y(), a.z() + b.z())});
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { a + (-b) });
impl_op_ex!(*|a: &Vec3, b: &f64| -> Vec3 { Vec3::from(a.x() * b, a.y() * b, a.z() * b) });
impl_op_ex!(*|a: &f64, b: &Vec3| -> Vec3 { b * a });
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::from(a.x() * b.x(), a.y() * b.y(), a.z() * b.z())
});
impl_op_ex!(/|a: &Vec3, b: &f64| -> Vec3 { a * (1.0 / b) });

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e: [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ],
    }
}

pub fn unit_vector(u: Vec3) -> Vec3 {
    u / u.length()
}

pub type Point3 = Vec3;
pub type Color = Vec3;
