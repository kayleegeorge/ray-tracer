
use crate::modules::vec3::Vec3;

use super::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }

    // Sets the hit record normal vector
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // The parameter `outward_normal` assume unit length
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

// Note: Hittable is a trait that can be implemented by any object that can be hit by a ray
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}