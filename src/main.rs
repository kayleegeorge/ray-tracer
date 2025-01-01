use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use raytracer::modules::camera::Camera;
use raytracer::modules::color::Color;
use raytracer::modules::hittable_list::HittableList;
use raytracer::modules::material::{Dielectric, Lambertian, Material, Metal};
use raytracer::modules::sphere::Sphere;
use raytracer::modules::utils::{random_double, random_double_range};
use raytracer::modules::vec3::{random, random_in_range, Point3, Vec3};

fn use_default_world() -> HittableList {
    let mut world = HittableList::new();

    // Make materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::new(1.5));
    let material_bubble   = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // Add a few objects to our world
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    world
}

fn generate_random_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Arc::new(ground));

    // Add many small random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new((a as f64) + 0.9 * random_double(), 0.2, (b as f64) + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // Diffuse
                    Arc::new(Lambertian::new(random()))
                } else if choose_mat < 0.95 {
                    // Metal
                    Arc::new(Metal::new(random_in_range(0.5, 1.0), random_double_range(0.0, 0.5)))
                } else {
                    // Dielectric glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    // Add a few large center spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn main() {
    // Create/open the output file
    let mut image_file = File::create("output/image.ppm").expect("Failed to create file");

    // Create world
    let world = generate_random_world();

    // Camera
    let mut camera = Camera::default();

    // Camera settings
    camera.aspect_ratio = 16.0 / 9.0;

    // DEFAULT WORLD CAMERA 
    // camera.image_width = 400;
    // camera.samples_per_pixel = 100;
    // camera.max_depth = 50;

    // camera.vfov = 90.0;
    // camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    // camera.lookat = Point3::new(0.0, 0.0, -1.0);
    // camera.vup = Vec3::new(0.0, 1.0, 0.0);

    // camera.defocus_angle = 3.0;
    // camera.focus_dist = 10.0;

    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    // Render the world 
    let image_string = camera.render(&world);
    writeln!(image_file, "{}", image_string).expect("Failed to write world image");
}