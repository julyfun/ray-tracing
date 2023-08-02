use crate::vec3;
use vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }
    pub fn dir(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
