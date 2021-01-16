use crate::hittable::{HitRecord, Hittable};
use crate::material::{DefaultMaterial, Material};
use crate::ray::Ray;
use crate::vec3;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: vec3::Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f64) -> Self {
        let mat_ptr = Rc::new(DefaultMaterial::new());
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !(t_min..=t_max).contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !(t_min..=t_max).contains(&root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
