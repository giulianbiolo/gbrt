// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the main function of the project
use gbrt::hittable::HitRecord;
use image::{ImageBuffer, Rgb};

mod vec3;
mod color;
mod point3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utility;
mod camera;
mod material;


use gbrt::vec3::{Color, Point3, Vec3};
use gbrt::ray::Ray;
use gbrt::hittable_list::HittableList;
use gbrt::sphere::Sphere;
use gbrt::hittable::Hittable;
use gbrt::camera::Camera;
use gbrt::material::{Lambertian, Metal};


fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth >= utility::MAX_DEPTH {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec: HitRecord = HitRecord::empty();
    if world.hit(r, 0.001, utility::INFINITY, &mut rec) {
        let mut scattered = Ray::empty();
        let mut attenuation = Color::empty();
        if rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth + 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0); // -1.0 < y < 1.0 => 0.0 < t < 1.0
    // Linear interpolation between white and blue gives us a gradient sky
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), std::fmt::Error> {

    // Print image size
    println!("Image Size: {}x{}", utility::WIDTH, utility::HEIGHT);

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

    // Camera
    let cam: Camera = Camera::new();

    // Render
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
    img.save("test.png").unwrap();
    Ok(())
}
