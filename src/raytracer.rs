// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various raytracing functions

use std::sync::{Arc, Mutex};
use core::sync::atomic::{AtomicU32, Ordering};

use likely_stable::unlikely;
use rayon::prelude::*;
use image::{ImageBuffer, Rgb};

use glam::Vec3A;

use crate::material::DiffuseLight;
use crate::ray::Ray;
use crate::hittable_list::HittableList;
use crate::hittable_list::Hittable;
use crate::sphere::Sphere;
use crate::mesh::Mesh;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::camera::Camera;
use crate::sphere_array::SphereArray;
use crate::utility;
use crate::utility::CONSTS;
use crate::color::{Color, to_rgb};
use crate::point3::Point3;
use crate::parser;


// Renders the scene to an image
#[allow(dead_code)]
pub fn render_to_image(world: &HittableList, cam: &Camera, filename: &str) {
    // Render function
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(CONSTS.width, CONSTS.height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..CONSTS.samples_per_pixel {
            let u: f32 = (x as f32 + utility::random_f32()) / (CONSTS.width - 1) as f32;
            let v: f32 = (CONSTS.height - y - 1) as f32 / (CONSTS.height - 1) as f32;
            let r: Ray = cam.get_ray(u, v);
            pixel_color = pixel_color + ray_color(&r, world, 0);
        }
        *pixel = to_rgb(pixel_color, CONSTS.samples_per_pixel);
    }
    // Save the image
    img.save(filename).unwrap();
}

pub fn render_to_image_multithreaded(world: &HittableList, cam: Camera, filename: &str) {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(CONSTS.width, CONSTS.height);
    let safe_img = Arc::new(Mutex::new(img));
    let safe_world = Arc::new(world.clone());
    let total_rows = CONSTS.height as f32;
    let completed_rows = AtomicU32::new(0);
    (0..CONSTS.height).into_par_iter().for_each(|y| {
        for x in 0..CONSTS.width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..CONSTS.samples_per_pixel {
                let u: f32 = (x as f32 + utility::random_f32()) / (CONSTS.width - 1) as f32;
                let v: f32 = (CONSTS.height - y as u32 - 1) as f32 / (CONSTS.height - 1) as f32;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &*safe_world, 0);
            }
            let rgb: Rgb<u8> = to_rgb(pixel_color, CONSTS.samples_per_pixel);
            let mut img = safe_img.lock().unwrap();
            img.put_pixel(x, y as u32, rgb);
        }
        completed_rows.fetch_add(1, Ordering::Relaxed);
        print!("{:.2}% complete\r", completed_rows.load(Ordering::Relaxed) as f32 / total_rows * 100.0);
    });
    // Save the image
    safe_img.lock().unwrap().save(filename).unwrap();
}

// Returns the color of a ray
pub fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if unlikely(depth >= CONSTS.max_depth) { return Color::new(0.0, 0.0, 0.0); }
    // Check for ray-sphere intersection
    if let Some(rec) = world.hit(r, utility::NEAR_ZERO, utility::INFINITY) {
        let mut scattered: Ray = Ray::empty();
        let mut attenuation: Vec3A = Vec3A::new(0.0, 0.0, 0.0);
        let emitted: Vec3A = rec.mat_ptr.emitted();
        if !rec.mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) { emitted }
        else { emitted + attenuation * ray_color(&scattered, world, depth + 1) }
    } else {
        // Return a skybox color
        Vec3A::ONE.lerp(utility::BLUE_SKY, 0.5 * (r.direction().normalize().y + 1.0))
    }
}

// Inits the scene and returns it as a HittableList
#[allow(dead_code)]
pub fn init_scene() -> HittableList {
    // Materials
    let material_ground: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let material_left: Metal = Metal::new(Color::new(0.3, 0.3, 0.3), 0.1);
    let material_right: Metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let material_high: DiffuseLight = DiffuseLight::new(Color::new(8.0, 8.0, 8.0));

    // World
    let mut world: HittableList = HittableList::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground), 0)));
    world.push(Box::new(Mesh::new(Point3::new(-1.0, 1.0, 8.0), 2.5, Vec3A::new(90.0, 90.0, 220.0), "models/jet/jet2.obj", Box::new(material_left))));
    world.push(Box::new(Sphere::new(Point3::new(1.5, 0.5, -1.0), 0.5, Box::new(material_right), 0)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 0.5, Box::new(material_high), 0)));
    _add_random_world_spheres(&mut world).expect("Failed to add random world spheres");
    world
}

#[allow(dead_code)]
pub fn init_random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    _add_random_world_spheres(&mut world).expect("Failed to add random world spheres");

    let mat1: Dielectric = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(mat1), 0)));
    let mat2: Lambertian = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(mat2), 0)));
    let mat3: Metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(mat3), 0)));

    let ground_material: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(ground_material), 0)));

    world
}

fn _add_random_world_spheres(world: &mut HittableList) -> Result<(), std::io::Error> {
    let mut spheres = Vec::<Sphere>::new();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = utility::random_f32();
            let center: Point3 = Vec3A::new(a as f32 + 0.9 * utility::random_f32(), 0.2, b as f32 + 0.9 * utility::random_f32());
            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo: Color = Vec3A::new(utility::random_f32(), utility::random_f32(), utility::random_f32()) * Vec3A::new(utility::random_f32(), utility::random_f32(), utility::random_f32());
                    let sphere_material: Lambertian = Lambertian::new(albedo);
                    spheres.push(Sphere::new(center, 0.2, Box::new(sphere_material), 0));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo: Color = Vec3A::new(utility::random_f32_range(0.5, 1.0), utility::random_f32_range(0.5, 1.0), utility::random_f32_range(0.5, 1.0));
                    let fuzz: f32 = utility::random_f32_range(0.0, 0.5);
                    let sphere_material: Metal = Metal::new(albedo, fuzz);
                    spheres.push(Sphere::new(center, 0.2, Box::new(sphere_material), 0));
                } else {
                    // Glass
                    let sphere_material: Dielectric = Dielectric::new(1.5);
                    spheres.push(Sphere::new(center, 0.2, Box::new(sphere_material), 0));
                }
            }
        }
    }
    let spheres_arr: SphereArray = SphereArray::new(&mut spheres);
    world.push(Box::new(spheres_arr));
    Ok(())
}

pub fn init_scene_from_yaml(filename: &str) -> HittableList { parser::parse_yaml_scene(filename) }
