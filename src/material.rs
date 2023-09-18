use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}
