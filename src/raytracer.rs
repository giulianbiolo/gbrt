// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various raytracing functions
use image::{ImageBuffer, Rgb};

use crate::vec3::{Color, Point3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::material::{Lambertian, Metal};
use crate::camera::Camera;
use crate::utility;



// Returns the color of a ray
pub fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth >= utility::MAX_DEPTH { return Color::new(0.0, 0.0, 0.0); }

    // Check for ray-sphere intersection
    let mut rec: HitRecord = HitRecord::empty();
    if world.hit(r, 0.001, utility::INFINITY, &mut rec) {
        let mut scattered = Ray::empty();
        let mut attenuation = Color::empty();
        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth + 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    // If no intersection, return the background sky color
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0); // -1.0 < y < 1.0 => 0.0 < t < 1.0
    // Linear interpolation between white and blue gives us a gradient sky
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

// Inits the scene and returns it as a HittableList
pub fn init_scene() -> HittableList {
    // Materials
    let material_ground: Lambertian = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center: Lambertian = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left: Metal = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right: Metal = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(material_ground))));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(material_center))));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Box::new(material_left))));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Box::new(material_right))));
    world
}

// Renders the scene to an image
pub fn render_to_image(world: &HittableList, cam: &Camera, filename: &str) {
    // Render function
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(utility::WIDTH, utility::HEIGHT);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..utility::SAMPLES_PER_PIXEL {
            let u: f32 = (x as f32 + utility::random_f32()) / (utility::WIDTH - 1) as f32;
            let v: f32 = (utility::HEIGHT - y - 1) as f32 / (utility::HEIGHT - 1) as f32;
            let r: Ray = cam.get_ray(u, v);
            pixel_color += ray_color(&r, &world, 0);
        }
        *pixel = pixel_color.to_rgb(utility::SAMPLES_PER_PIXEL);
    }

    // Save the image
    img.save(filename).unwrap();
}
