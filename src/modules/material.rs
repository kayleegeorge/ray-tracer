/*
 * Material
 * 
 * Needs to be able to:
 * 1. Produce a scattered ray (or say it absorbed the incident ray).
 * 2. If scattered, say how much the ray should be attenuated.
 */

use super::{color::Color, hittable::HitRecord, ray::Ray, utils::random_double, vec3::{random_unit_vector, reflect, refract, Vec3}};

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

/*
 * Dielectric is like glass. It refracts light.
 * 
 * The refraction index is the ratio of the material's refractive index over the refractive index of the enclosing medium.
 */
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    // Real glass has reflectivity that varies with angle of incidence.
    // So we use Schlick's polynomial approximation for reflectance (Fresnel equation)
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let r = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = r_in.direction().unit_vector();
        let cos_theta = f64::min(unit_direction.dot(&-rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        // Total internal reflection
        let cannot_refract = r * sin_theta > 1.0;
        let direction = if cannot_refract || self.reflectance(cos_theta, r) > random_double() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, r)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}