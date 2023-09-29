use ray_tracing::{
    camera, hittable_list,
    material::{Dielectric, FuzzyMetal, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};
use std::rc::Rc;

fn main() {
    let mut world = hittable_list::HittableList::new();
    let ground = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::from(Color::from(0.1, 0.2, 0.5)));
    let left = Rc::new(Dielectric::from(1.5));
    let right = Rc::new(FuzzyMetal::from(Color::from(0.8, 0.6, 0.2), 0.0));
    world.add(Rc::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        ground,
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
        center,
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        0.5,
        left.clone(),
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        -0.4,
        left,
    )));
    world.add(Rc::new(Sphere::from(
        Point3::from(1.0, 0.0, -1.0),
        0.5,
        right,
    )));

    let cam = camera::Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20_f64.to_radians(),
        Point3::from(-2.0, 2.0, 1.0),
        Point3::from(0.0, 0.0, -1.0),
        Vec3::from(0.0, 1.0, 0.0),
    );
    cam.render(&world);
}
