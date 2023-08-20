mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use rand::Rng;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn main() {
    let mut world = hittable_list::HittableList::new();
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(0.0, -80.5, -1.0),
        100.0,
    )));

    let cam = camera::Camera::new(16.0 / 9.0, 400, 100);
    cam.render(&world);
}
