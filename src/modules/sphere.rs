use std::sync::Arc;

use super::{hittable::{HitRecord, Hittable}, interval::Interval, material::Material, ray::Ray, vec3::Point3};


pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self { center, radius: f64::max(radius, 0.0), mat }
    }
}

// Implements the Hittable trait for Sphere objects 
impl Hittable for Sphere {

    /*
     * Ray-sphere intersection
     * 
     * Solving for t in the ray equation: r(t) = origin + t * direction
     * 
     * If the discriminant is positive, two solutions for t (i.e. two intersections with the sphere)
     * If the discriminant is zero, one solution for t (i.e. one intersection with the sphere)
     * If the discriminant is negative, no solutions for t (i.e. no intersections with the sphere)
     * 
     * This allows us to determine whether the ray intersects the sphere and where
     */
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        
        let sqrt_d = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
}