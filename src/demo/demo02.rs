use std::fs::File;
use std::io;
use std::io::Write;

use crate::ray::Ray;
use crate::vec3;

const FILENAME: &str = "pic/02.ppm";

// 线性插值
fn lerp(t: f64, start: vec3::Color, end: vec3::Color) -> vec3::Color {
    (1.0 - t) * start + t * end
}

// 实现渐变色
fn ray_color(r: Ray) -> vec3::Color {
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

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3 {
        0: 0.0,
        1: 0.0,
        2: 0.0,
    }; // 眼睛位置
    let horizontal = vec3::Vec3(viewport_width, 0.0, 0.0); // 屏幕水平宽度
    let vertical = vec3::Vec3(0.0, viewport_height, 0.0); // 屏幕垂直高度
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3(0.0, 0.0, focal_length); //视口左下角作为开始位置

    // Render
    let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut f = File::create(FILENAME)?;
    f.write_all(part0.as_bytes())?;

    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", row);
        for col in 0..image_width {
            let row = row as f64;
            let col = col as f64;
            let width = (image_width - 1) as f64;
            let height = (image_height - 1) as f64;

            let u = col / width;
            let v = row / height;
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray { origin, direction };

            let pixel_color = ray_color(r);
            pixel_color.write_color(&mut f, 1)?;
        }
    }

    eprintln!("\nDone.");
    Ok(())
}
