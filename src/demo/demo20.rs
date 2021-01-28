use std::fs::File;
use std::io;
use std::io::Write;
use std::rc::Rc;

use crate::camera::LensCamera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils;
use crate::vec3;

const FILENAME: &str = "pic/20.ppm";

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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let mut ground = Sphere::new(
        vec3::Point3 {
            0: 0.0,
            1: -1000.0,
            2: 0.0,
        },
        1000.0,
    );
    let ground_material = Rc::new(Lambertian::new(vec3::Color {
        0: 0.5,
        1: 0.5,
        2: 0.5,
    }));
    ground.mat_ptr = ground_material;

    world.add(Rc::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = utils::random();
            let center = vec3::Point3 {
                0: a + 0.9 * utils::random(),
                1: 0.2,
                2: b + 0.9 * utils::random(),
            };

            let point = vec3::Point3 {
                0: 4.0,
                1: 0.2,
                2: 0.0,
            };

            if (center - point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::Color::random() * vec3::Color::random();
                    let sphere_material = Rc::new(Lambertian::new(albedo));
                    let mut sphere = Sphere::new(center, 0.2);
                    sphere.mat_ptr = sphere_material;
                    world.add(Rc::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::Color::random_in(0.5, 1.0);
                    let fuzz = utils::random_in(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    let mut sphere = Sphere::new(center, 0.2);
                    sphere.mat_ptr = sphere_material;
                    world.add(Rc::new(sphere));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    let mut sphere = Sphere::new(center, 0.2);
                    sphere.mat_ptr = sphere_material;
                    world.add(Rc::new(sphere));
                }
            }
        }
    }
    let material1 = Rc::new(Dielectric::new(1.5));
    let mut sphere1 = Sphere::new(
        vec3::Point3 {
            0: 0.0,
            1: 1.0,
            2: 0.0,
        },
        1.0,
    );
    sphere1.mat_ptr = material1;
    world.add(Rc::new(sphere1));

    let material2 = Rc::new(Lambertian::new(vec3::Color {
        0: 0.4,
        1: 0.2,
        2: 0.1,
    }));
    let mut sphere2 = Sphere::new(
        vec3::Point3 {
            0: -4.0,
            1: 1.0,
            2: 0.0,
        },
        1.0,
    );
    sphere2.mat_ptr = material2;
    world.add(Rc::new(sphere2));

    let material3 = Rc::new(Metal::new(
        vec3::Color {
            0: 0.7,
            1: 0.6,
            2: 0.5,
        },
        0.0,
    ));
    let mut sphere3 = Sphere::new(
        vec3::Point3 {
            0: 4.0,
            1: 1.0,
            2: 0.0,
        },
        1.0,
    );
    sphere3.mat_ptr = material3;
    world.add(Rc::new(sphere3));

    world
}

pub fn run() -> io::Result<()> {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = vec3::Point3 {
        0: 13.0,
        1: 2.0,
        2: 3.0,
    };
    let lookat = vec3::Point3 {
        0: 0.0,
        1: 0.0,
        2: 0.0,
    };
    let vup = vec3::Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = LensCamera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

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
