/*
 * Ray.rs
 * 
 * A ray is defined by an origin and a direction: P(t) = A + tb.
 * 
 * The ray is parameterized by a real number t, which is the distance along the ray,
 * and is defined by the origin A and the direction b - A.
 */

use crate::modules::vec3::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn default() -> Self {
        Self { origin: Vec3::new(0.0, 0.0, 0.0), direction: Vec3::new(0.0, 0.0, 0.0) }
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    // Getters
    pub fn origin(&self) -> Vec3 { self.origin }
    pub fn direction(&self) -> Vec3 { self.direction }

    // Returns the point at parameter t along the ray
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}