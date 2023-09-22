use crate::vec3;
use vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
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
    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }
}
