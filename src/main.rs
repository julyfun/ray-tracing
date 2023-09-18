use rand::Rng;
use ray_tracing::camera;
use ray_tracing::hittable::Hittable;
use ray_tracing::hittable_list;
use ray_tracing::sphere;
use ray_tracing::vec3::Point3;

fn main() {
    let mut world = hittable_list::HittableList::new();
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(std::rc::Rc::new(sphere::Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = camera::Camera::new(16.0 / 9.0, 400, 100, 50);
    cam.render(&world);
}
