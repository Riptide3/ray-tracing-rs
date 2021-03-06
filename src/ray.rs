use crate::vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: vec3::Point3,
    pub direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Point3, direction: vec3::Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + t * self.direction
    }
}
