// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various raytracing functions
use std::sync::{Arc, Mutex};
use std::thread;
use image::{ImageBuffer, Rgb};
use glam;
use glam::vec3a;

use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::camera::Camera;
use crate::utility;
use crate::color::{Color, to_rgb};
use crate::point3::Point3;



// Renders the scene to an image
#[allow(dead_code)]
pub fn render_to_image(world: &HittableList<Sphere>, cam: &Camera, filename: &str) {
    // Render function
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(utility::WIDTH, utility::HEIGHT);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..utility::SAMPLES_PER_PIXEL {
            let u: f32 = (x as f32 + utility::random_f32()) / (utility::WIDTH - 1) as f32;
            let v: f32 = (utility::HEIGHT - y - 1) as f32 / (utility::HEIGHT - 1) as f32;
            let r: Ray = cam.get_ray(u, v);
            pixel_color = pixel_color + ray_color(&r, world, 0);
        }
        *pixel = to_rgb(pixel_color, utility::SAMPLES_PER_PIXEL);
    }

    // Save the image
    img.save(filename).unwrap();
}

pub fn render_to_image_multithreaded(world: &HittableList<Sphere>, cam: Camera, filename: &str) {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(utility::WIDTH, utility::HEIGHT);
    let rows_per_thread = utility::HEIGHT / utility::THREADS;
    let safe_img = Arc::new(Mutex::new(img));

    let mut thread_handles = vec![];
    for i in 0..utility::THREADS {
        // let world = init_random_scene();
        let safe_img = Arc::clone(&safe_img);
        let start_row = i * rows_per_thread;
        let end_row = if i == utility::THREADS - 1 { utility::HEIGHT } else { (i + 1) * rows_per_thread };
        let world = world.clone();

        thread_handles.push(thread::spawn(move || {
            for y in start_row..end_row {
                for x in 0..utility::WIDTH {
                    let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
                    for _s in 0..utility::SAMPLES_PER_PIXEL {
                        let u: f32 = (x as f32 + utility::random_f32()) / (utility::WIDTH - 1) as f32;
                        let v: f32 = (utility::HEIGHT - y - 1) as f32 / (utility::HEIGHT - 1) as f32;
                        let r: Ray = cam.get_ray(u, v);
                        pixel_color = pixel_color + ray_color(&r, &world, 0);
                    }
                    let rgb = to_rgb(pixel_color, utility::SAMPLES_PER_PIXEL);
                    let mut img = safe_img.lock().unwrap();
                    img.put_pixel(x, y, rgb);
                }
            }
        }));
    }
    for handle in thread_handles { handle.join().unwrap(); }
    // Save the image
    safe_img.lock().unwrap().save(filename).unwrap();
}

// Returns the color of a ray
pub fn ray_color(r: &Ray, world: &HittableList<Sphere>, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth >= utility::MAX_DEPTH { return Color::new(0.0, 0.0, 0.0); }

    // Check for ray-sphere intersection
    let mut rec: HitRecord = HitRecord::empty();
    if world.hit(r, 0.001, utility::INFINITY, &mut rec) {
        let mut scattered = Ray::empty();
        let mut attenuation = vec3a(0.0, 0.0, 0.0);
        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth + 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    // If no intersection, return the background sky color
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0); // -1.0 < y < 1.0 => 0.0 < t < 1.0
    // Linear interpolation between white and blue gives us a gradient sky
    // Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    Color::new(1.0, 1.0, 1.0).lerp(Color::new(0.5, 0.7, 1.0), t)
}

// Inits the scene and returns it as a HittableList
#[allow(dead_code)]
pub fn init_scene() -> HittableList<Sphere> {
    // Materials
    let material_ground: Lambertian = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center: Lambertian = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left: Dielectric = Dielectric::new(1.5);
    let material_right: Metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // World
    let mut world: HittableList<Sphere> = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(material_ground)));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(material_center)));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Box::new(material_left.clone())));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.45, Box::new(material_left)));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Box::new(material_right)));
    world
}

#[allow(dead_code)]
pub fn init_random_scene() -> HittableList<Sphere> {
    let ground_material: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let mut world: HittableList<Sphere> = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(ground_material)));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = utility::random_f32();
            let center: Point3 = vec3a(a as f32 + 0.9 * utility::random_f32(), 0.2, b as f32 + 0.9 * utility::random_f32());
            if (center - vec3a(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Color = vec3a(utility::random_f32(), utility::random_f32(), utility::random_f32()) * vec3a(utility::random_f32(), utility::random_f32(), utility::random_f32());
                    let sphere_material: Lambertian = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Color = vec3a(utility::random_f32_range(0.5, 1.0), utility::random_f32_range(0.5, 1.0), utility::random_f32_range(0.5, 1.0));
                    let fuzz: f32 = utility::random_f32_range(0.0, 0.5);
                    let sphere_material: Metal = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                } else {
                    // Glass
                    let sphere_material: Dielectric = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                }
            }
        }
    }
    let mat1: Dielectric = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(mat1)));
    let mat2: Lambertian = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(mat2)));
    let mat3: Metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(mat3)));

    world
}
