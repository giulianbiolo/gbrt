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

use gbrt::vec3::{Color, Point3};
use gbrt::ray::Ray;
use gbrt::hittable_list::HittableList;
use gbrt::sphere::Sphere;
use gbrt::hittable::Hittable;
use gbrt::camera::Camera;


fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec: HitRecord = HitRecord::empty();
    if world.hit(r, 0.0, utility::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0); // -1.0 < y < 1.0 => 0.0 < t < 1.0
    // Linear interpolation between white and blue gives us a gradient sky
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), std::fmt::Error> {
    println!("Image Size: {}x{}", utility::WIDTH, utility::HEIGHT);
    // World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            pixel_color += ray_color(&r, &world);
        }
        *pixel = pixel_color.to_rgb(utility::SAMPLES_PER_PIXEL);
    }
    img.save("test.png").unwrap();
    Ok(())
}
