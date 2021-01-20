use crate::ray::Ray;
use crate::vec3;

pub struct Camera {
    pub origin: vec3::Point3,
    pub lower_left_corner: vec3::Point3,
    pub horizontal: vec3::Vec3,
    pub vertical: vec3::Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = vec3::Point3 {
            0: 0.0,
            1: 0.0,
            2: 0.0,
        };
        let horizontal = vec3::Vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3::Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}

pub struct AdjustableFOVCamera {
    pub origin: vec3::Point3,
    pub lower_left_corner: vec3::Point3,
    pub horizontal: vec3::Vec3,
    pub vertical: vec3::Vec3,
}

impl AdjustableFOVCamera {
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = vec3::Point3 {
            0: 0.0,
            1: 0.0,
            2: 0.0,
        };
        let horizontal = vec3::Vec3(viewport_width, 0.0, 0.0);
        let vertical = vec3::Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3(0.0, 0.0, focal_length);

        AdjustableFOVCamera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}

pub struct PositionableCamera {
    pub origin: vec3::Point3,
    pub lower_left_corner: vec3::Point3,
    pub horizontal: vec3::Vec3,
    pub vertical: vec3::Vec3,
}

impl PositionableCamera {
    pub fn new(
        lookfrom: vec3::Point3,
        lookat: vec3::Point3,
        vup: vec3::Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        PositionableCamera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
