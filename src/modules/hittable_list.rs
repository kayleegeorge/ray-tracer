use std::sync::Arc;

use super::{hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray};

/*
 * Box vs. Arc:
 * Box: Ownership of a value (single owner)
 * Arc: Atomic Reference Counting allows shared ownership to some value (ie.g. access same var from multiple threads)
 * 
 * We use Arc because it allows multiple geometries to share a common instance (e.g. spheres with the same color)
 */
pub struct HittableList {
    // Vector (dynamic array) of shared pointers (ref-counted) to hittable objects
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}