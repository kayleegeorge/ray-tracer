use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use raytracer::modules::camera::Camera;
use raytracer::modules::hittable_list::HittableList;
use raytracer::modules::sphere::Sphere;
use raytracer::modules::vec3::Point3;

fn main() {
    // Create/open the output file
    let mut image_file = File::create("output/image.ppm").expect("Failed to create file");

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    // Render the world 
    let image_string = camera.render(&world);
    writeln!(image_file, "{}", image_string).expect("Failed to write world image");
}