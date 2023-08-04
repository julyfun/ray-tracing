mod color;
mod ray;
mod vec3;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    // 我们来看看是什么玩意
    let unit_direction = vec3::unit_vector(r.direction());
    // y 分量是三维几何的 y，朝上
    // t 随着 y 分量从 -1 到 1，由 0.0 到 1.0
    let t = 0.5 + 0.5 * unit_direction.y();
    // 图像上方是纯白，下方是淡蓝
    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // 这个单词就是屏幕比的意思
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // -1 到 1
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("{} lines remaining..", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            // j 反向枚举，这样 v （三维纵坐标）也是反向的，从最高点到最低点
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::from(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel_color = ray_color(&r);
            color::write_color(&pixel_color);
        }
    }
}
