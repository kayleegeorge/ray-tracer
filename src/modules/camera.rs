use crate::modules::color::write_color;

use super::{color::Color, hittable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, ray::Ray, utils::{random_double, INFINITY}, vec3::{Point3, Vec3}};


pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width to height
    pub image_width: u32, // Rendered image width in pixels
    pub samples_per_pixel: u32, // Number of samples per pixel
    pub max_depth: u32, // Maximum number of bounces a ray can make

    image_height: u32, // Rendered image height in pixels
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3, // Camera center
    pixel00_loc: Point3, // Location of 0,0 pixel
    pixel_delta_u: Vec3, // Offset from the center of the pixel to the right
    pixel_delta_v: Vec3, // Offset from the center of the pixel to the bottom
}

impl Camera {
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }

    pub fn render(&mut self, world: &HittableList) -> String {
        self.init();
        let mut image_string = String::new();

        // Write PPM header
        image_string.push_str(&format!("P3\n{} {}\n255\n", self.image_width, self.image_height));

        // Render PPM image
        for j in 0..self.image_height {
            // Note: eprint goes to stderr instead of stdout
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                // Write color to the image string output with newline
                let color_string = write_color(pixel_color * self.pixel_samples_scale);
                image_string.push_str(&color_string);
                image_string.push('\n');
            }
        }
        eprintln!("Done.\n");

        return image_string;
    }

    fn init(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height == 0 { 1 } else { self.image_height }; // Ensure at least 1 pixel

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::zero();

        // Define viewport
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calcuate the horizontal and vertical delta vectors from pizel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    /* 
     * Returns the color for a given scene ray
     * 
     * Linear gradient (linear interpolation); "lerp" between two values: (1 - a) * start + a * end
     * where a: 0 -> 1
     * 
     * Color diffusion:
     * If a ray bounces off a material and keeps 100% of its color, then it's white.
     * If a ray bounces off a material and keeps 0% of its color, then it's black.
     */
    fn ray_color<T: Hittable>(&self, r: &Ray, depth: u32, world: &T) -> Color {
        // No more light gathered if max ray bounce depth is reached
        if depth <= 0 {
            return Color::zero();
        }

        let mut rec = HitRecord::default();
        // A ray attemps to accurately calculate the intersection point when intersecting with a hittable
        // Someones this calculation is not accurate (floating point rounding error) so we add a small epsilon
        // This fixes the "shadow acne" problem
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let direction = rec.normal.random_on_hemisphere(rec.normal);
            return 0.5 * (self.ray_color(&Ray::new(rec.p, direction), depth - 1, world));
        }
        
        // If no hit, keep the sky gradient
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /*
     * Constructs a camera ray originating from the camera center and passing through the pixel at (i, j)
     * Randomly samples the pixel to account for antialiasing
     */
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.pixel_sample_square();

        // Randomly sample a pixel center
        let pixel_center = self.pixel00_loc + 
            ((i as f64 + offset.x()) * self.pixel_delta_u) + 
            ((j as f64 + offset.y()) * self.pixel_delta_v);   
        let ray_direction = pixel_center - self.center;
        Ray::new(self.center, ray_direction)
    }

    /*
     * Returns a random point in the [-.5,-.5]-[+.5,+.5] unit square centered on the pixel
     */
    fn pixel_sample_square(&self) -> Vec3 {
        return Vec3::new(-0.5 + random_double(), -0.5 + random_double(), 0.0);
    }
}