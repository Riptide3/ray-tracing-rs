use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::Write;
use std::sync::Arc;

use crate::camera::AdjustableFOVCamera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Lambertian;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils;
use crate::vec3;

const FILENAME: &str = "pic/16.ppm";

// 线性插值
fn lerp(t: f64, start: vec3::Color, end: vec3::Color) -> vec3::Color {
    (1.0 - t) * start + t * end
}

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: u64) -> vec3::Color {
    if depth == 0 {
        return vec3::Color {
            0: 0.0,
            1: 0.0,
            2: 0.0,
        };
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(vec3::Point3::fill(0.0), vec3::Vec3::fill(0.0));
        let mut attenuation = vec3::Vec3::fill(0.0);
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return vec3::Color::fill(0.0);
        }
    }

    let unit_direction = r.direction.unit_vector(); // 单位化
    let t = 0.5 * (unit_direction.y() + 1.0); // 将y分量映射到[0, 1]

    let from = vec3::Color {
        0: 1.0,
        1: 1.0,
        2: 1.0,
    }; // 白色
    let to = vec3::Color {
        0: 0.5,
        1: 0.7,
        2: 1.0,
    }; // 蓝色

    lerp(t, from, to)
}

pub fn run() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let r = (PI / 4.0).cos();
    let mut world = HittableList::new();

    let mut sphere_0 = Sphere::new(
        vec3::Point3 {
            0: -r,
            1: 0.0,
            2: -1.0,
        },
        r,
    );
    let mut sphere_1 = Sphere::new(
        vec3::Point3 {
            0: r,
            1: 0.0,
            2: -1.0,
        },
        r,
    );

    let material_left = Arc::new(Lambertian::new(vec3::Color {
        0: 0.0,
        1: 0.0,
        2: 1.0,
    }));
    let material_right = Arc::new(Lambertian::new(vec3::Color {
        0: 1.0,
        1: 0.0,
        2: 0.0,
    }));

    sphere_0.mat_ptr = material_left;
    sphere_1.mat_ptr = material_right;

    world.add(Arc::new(sphere_0));
    world.add(Arc::new(sphere_1));

    // Camera
    let cam = AdjustableFOVCamera::new(90.0, aspect_ratio);

    // Render
    let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut f = File::create(FILENAME)?;
    f.write_all(part0.as_bytes())?;

    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", row);
        for col in 0..image_width {
            let mut pixel_color = vec3::Color {
                0: 0.0,
                1: 0.0,
                2: 0.0,
            };
            for _ in 0..samples_per_pixel {
                let u = (col as f64 + utils::random()) / (image_width - 1) as f64;
                let v = (row as f64 + utils::random()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            pixel_color.write_color(&mut f, samples_per_pixel)?;
        }
    }

    eprintln!("\nDone.");
    Ok(())
}
