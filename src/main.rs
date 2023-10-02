use rand::Rng;
use ray_tracing::{
    camera, hittable_list,
    material::{Dielectric, FuzzyMetal, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};
use std::rc::Rc;

fn main() {
    let mut world = hittable_list::HittableList::new();
    let mat_ground = Rc::new(Lambertian::from(Color::from(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));
    let gen = || rand::thread_rng().gen();
    let gen_range = |x, y| rand::thread_rng().gen_range(x..y);
    for a in -11..11 {
        for b in -11..11 {
            let mat_num: f64 = gen();
            let center = Point3::from(a as f64 + 0.9 * gen(), 0.2, b as f64 + 0.9 * gen());
            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat_num < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let mat = Rc::new(Lambertian::from(albedo));
                    world.add(Rc::new(Sphere::from(center, 0.2, mat)));
                } else if mat_num < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = gen_range(0.0, 0.5);
                    let fuzz_mat = Rc::new(FuzzyMetal::from(albedo, fuzz));
                    world.add(Rc::new(Sphere::from(center, 0.2, fuzz_mat)));
                } else {
                    let mat = Rc::new(Dielectric::from(1.5));
                    world.add(Rc::new(Sphere::from(center, 0.2, mat)));
                }
            }
        }
    }
    let mat = Rc::new(Dielectric::from(1.5));
    world.add(Rc::new(Sphere::from(Point3::from(0.0, 1.0, 0.0), 1.0, mat)));

    let mat = Rc::new(Lambertian::from(Color::from(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::from(
        Point3::from(-4.0, 1.0, 0.0),
        1.0,
        mat,
    )));
    let mat = Rc::new(FuzzyMetal::from(Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::from(Point3::from(4.0, 1.0, 0.0), 1.0, mat)));
    let cam = camera::Camera::from(
        16.0 / 9.0,
        1200,
        500,
        50,
        20f64.to_radians(),
        Point3::from(13.0, 2.0, 3.0),
        Point3::from(0.0, 0.0, 0.0),
        Vec3::from(0.0, 1.0, 0.0),
        0.6f64.to_radians(),
        10.0,
    );
    cam.render(&world);
}
