use crate::material::{DefaultMaterial, Material};
use crate::ray::Ray;
use crate::vec3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: vec3::Point3,    //交点
    pub normal: vec3::Vec3, //法向量
    pub t: f64,             //距离
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: vec3::Point3 {
                0: 0.0,
                1: 0.0,
                2: 0.0,
            },
            normal: vec3::Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat_ptr: Arc::new(DefaultMaterial::new()),
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: vec3::Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
