use rand::Rng;

use crate::color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list;
use crate::interval;
use crate::ray::{self, Ray};
use crate::sphere;
use crate::vec3::{self, Color, Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    samples_per_pixel: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Self {
        // 这个单词就是屏幕比的意思
        let image_height = std::cmp::max((image_width as f64 / aspect_ratio) as i32, 1);

        // -1 到 1
        // [相机]
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let camera_center = Point3::from(0.0, 0.0, 0.0);

        // [计算空间中相机投影平面的向量]
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

        // [计算每个像素在空间中的向量]
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // [空间中相机投影片面的左上角位置]
        let viewport_upper_left = camera_center
            - Vec3::from(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        // [像素点 00 的中心点] # 像素点是一个正方形
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color<T>(r: &Ray, world: &T) -> Color
    where
        T: Hittable,
    {
        // 呃这是一个直接覆盖在屏幕上的球球
        // 这个 z 轴正负是个什么玩意
        // 我们来看看是什么玩意
        let (hit, rec) = world.hit(r, interval::Interval::from(0.0, f64::INFINITY));
        // 颜色取决于光线与物体的碰撞平面方向
        if hit {
            return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
        }
        let unit_direction = vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
    }

    pub fn render<T>(&self, world: &T)
    where
        T: Hittable,
    {
        let mut stdout = std::io::stdout();
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprintln!("{} lines remaining..", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = vec3::Color::from(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world);
                }
                // eprintln!("{}", pixel_color);
                if let Err(err) =
                    color::write_color(&mut stdout, &pixel_color, self.samples_per_pixel)
                {
                    eprintln!("Errors occurred! {}", err);
                }
            }
        }
        eprintln!("Done");
    }

    // 返回一个像素点内部的随机位置
    fn pixel_sample_square(&self) -> vec3::Vec3 {
        let px = -0.5 + rand::random::<f64>();
        let py = -0.5 + rand::random::<f64>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    // 输入像素坐标，返回对应像素方形内随机一条光线
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        // center 就是 0 0 0
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::from(ray_origin, ray_direction)
    }
}
