/*
 * Material
 * 
 * Needs to be able to:
 * 1. Produce a scattered ray (or say it absorbed the incident ray).
 * 2. If scattered, say how much the ray should be attenuated.
 */

use super::{color::Color, hittable::HitRecord, ray::Ray, vec3::{random_unit_vector, reflect, Vec3}};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

/*
 * Lambertian (diffuse) reflection for modeling light attenuation.
 * Can either always scatter, sometimes scatter, or scatter with some probability.
 */
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

/*
 * Fuzzy Reflection 
 * 
 * Uses a small sphere to randomize the reflected direction. The fuzziness param is the radius of the sphere.
 * Fuzz needs to be consistently scaled to the reflection vector so we need to normalize the reflected ray. 
 */
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz: f64::min(fuzz, 1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = reflected.unit_vector() + self.fuzz * random_unit_vector(); // Add fuzz to the reflected ray

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}