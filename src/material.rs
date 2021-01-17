use std::fmt::Debug;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3;
use std::rc::Rc;

pub trait Material: Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut vec3::Color,
        scattered: &mut Ray,
    ) -> bool;
    fn rc_clone(&self) -> Rc<dyn Material>;
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultMaterial {}

impl DefaultMaterial {
    pub fn new() -> Self {
        DefaultMaterial {}
    }
}

impl Material for DefaultMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut vec3::Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn rc_clone(&self) -> Rc<dyn Material> {
        Rc::new(DefaultMaterial::new())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: vec3::Color,
}

impl Lambertian {
    pub fn new(albedo: vec3::Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut vec3::Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + vec3::Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
    fn rc_clone(&self) -> Rc<dyn Material> {
        Rc::new(Lambertian::new(self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: vec3::Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: vec3::Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut vec3::Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::Vec3::reflect(r_in.direction.unit_vector(), rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * vec3::Vec3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        scattered.direction.dot(rec.normal) > 0.0
    }
    fn rc_clone(&self) -> Rc<dyn Material> {
        Rc::new(Metal::new(self.albedo, 0.0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut vec3::Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = vec3::Color::fill(1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec3::Vec3::unit_vector(&r_in.direction);
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            vec3::Vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
    fn rc_clone(&self) -> Rc<dyn Material> {
        Rc::new(Dielectric::new(self.ir))
    }
}
