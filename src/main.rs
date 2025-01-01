use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use raytracer::modules::camera::Camera;
use raytracer::modules::color::Color;
use raytracer::modules::hittable_list::HittableList;
use raytracer::modules::material::{Dielectric, Lambertian, Metal};
use raytracer::modules::sphere::Sphere;
use raytracer::modules::vec3::{Point3, Vec3};

fn main() {
    // Create/open the output file
    let mut image_file = File::create("output/image.ppm").expect("Failed to create file");

    // World
    let mut world = HittableList::new();

    // Make materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::new(1.5));
    let material_bubble   = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // Add objects to our world
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 90.0;
    camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 3.0;
    camera.focus_dist = 10.0;


    // Render the world 
    let image_string = camera.render(&world);
    writeln!(image_file, "{}", image_string).expect("Failed to write world image");
}