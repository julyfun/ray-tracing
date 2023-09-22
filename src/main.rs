use ray_tracing::{
    camera, hittable_list,
    material::{FuzzyMetal, Lambertian, Metal},
    sphere,
    vec3::{Color, Point3},
};
use std::rc::Rc;

fn main() {
    let ground = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::from(Color::from(0.7, 0.3, 0.3)));
    let left = Rc::new(FuzzyMetal::from(Color::from(0.8, 0.8, 0.8), 0.1));
    let right = Rc::new(FuzzyMetal::from(Color::from(0.8, 0.8, 0.8), 0.5));
    let mut world = hittable_list::HittableList::new();
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
        center,
    )));
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        0.5,
        left,
    )));
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(1.0, 0.0, -1.0),
        0.5,
        right,
    )));
    let cam = camera::Camera::new(16.0 / 9.0, 400, 100, 50);
    cam.render(&world);
}
