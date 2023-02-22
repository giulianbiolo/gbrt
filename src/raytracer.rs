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
use crate::material::ScatterRecord;
use crate::ray::Ray;
use crate::hittable_list::HittableList;
use crate::hittable_list::Hittable;
use crate::sphere::Sphere;
use crate::mesh::Mesh;
use crate::material::{Lambertian, Metal, Dielectric};
use crate::camera::Camera;
use crate::sphere_array::SphereArray;
use crate::utility;
use crate::utility::{CONSTS, random_f32, load_environment, load_filter};
use crate::color::{Color, to_rgb};
use crate::point3::Point3;
use crate::parser;
use crate::sampling_filters::Filter;
use crate::pdf::{PDF, HittablePDF, MixturePDF};


// Renders the scene to an image
#[allow(dead_code)]
pub fn render_to_image(world: &HittableList, cam: &Camera, filename: &str) {
    // Render function
    let envmap: Arc<dyn Hittable + Send + Sync> = load_environment();
    let lights = if get_lights(world).len() > 0 { get_lights(world) } else { Vec::from([envmap.clone()]) };
    let filter: Box<dyn Filter + Send + Sync> = load_filter();
    println!("Chosen Filter: {}", filter);
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(CONSTS.width, CONSTS.height);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..CONSTS.samples_per_pixel {
            let u: f32 = (x as f32 + filter.sample(random_f32())) / (CONSTS.width as f32 - 1.0);
            let v: f32 = (CONSTS.height as f32 - (y as f32 + filter.sample(random_f32()))) as f32 / (CONSTS.height as f32 - 1.0);
            let r: Ray = cam.get_ray(u, v);
            pixel_color = pixel_color + ray_color(&r, world, &lights, &envmap, 0);
        }
        *pixel = to_rgb(pixel_color, CONSTS.samples_per_pixel as f32);
    }
    // Save the image
    img.save(filename).unwrap();
}

pub fn render_to_image_multithreaded(world: &HittableList, cam: Camera, filename: &str) {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(CONSTS.width, CONSTS.height);
    let safe_img = Arc::new(Mutex::new(img));
    let safe_world = Arc::new(world.clone());
    let environment_map: Arc<dyn Hittable + Send + Sync> = load_environment();
    let lights = if get_lights(world).len() > 0 { get_lights(world) } else { Vec::from([environment_map.clone()]) };
    let filter: Box<dyn Filter + Send + Sync> = load_filter();
    println!("Chosen Filter: {}", filter);

    let total_rows = CONSTS.height as f32;
    let completed_rows = AtomicU32::new(0);
    (0..CONSTS.height).into_par_iter().for_each(|y| {
        for x in 0..CONSTS.width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..CONSTS.samples_per_pixel {
                let u: f32 = (x as f32 + filter.sample(random_f32())) / (CONSTS.width as f32 - 1.0);
                let v: f32 = (CONSTS.height as f32 - (y as f32 + filter.sample(random_f32()))) / (CONSTS.height as f32 - 1.0);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &*safe_world, &lights, &environment_map, 0);
            }
            let rgb: Rgb<u8> = to_rgb(pixel_color, CONSTS.samples_per_pixel as f32);
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
pub fn ray_color(r: &Ray, world: &HittableList, lights: &HittableList, envmap: &Arc<dyn Hittable + Sync + Send>, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if unlikely(depth >= CONSTS.max_depth) { return Color::new(0.0, 0.0, 0.0); }
    // Check for ray-sphere intersection
    if let Some(rec) = world.hit(r, utility::NEAR_ZERO, utility::INFINITY) {
        let mut srec = ScatterRecord::new();
        let emitted: Vec3A = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
        if !rec.mat_ptr.scatter(r, &rec, &mut srec) { emitted }
        else {
            // Russian roulette
            if depth > utility::CONSTS.min_depth && utility::random_f32() < srec.attenuation.max_element() { emitted }
            else {
                // If the material is specular, we can just return the color of the specular ray
                if srec.is_specular { return srec.attenuation * ray_color(&srec.specular_ray, world, lights, envmap, depth + 1); }
                // We hit an object, so we need to compute the light
                let light_pdf: HittablePDF = HittablePDF::new(rec.p, Arc::new(lights.clone()));
                let mut scattered: Ray;
                let pdf: f32;
                if srec.pdf_ptr.is_some() {
                    let mixture_pdf: MixturePDF = MixturePDF::new(srec.pdf_ptr.unwrap(), Arc::new(light_pdf));
                    scattered = Ray::new(rec.p, mixture_pdf.generate().normalize());
                    pdf = mixture_pdf.value(&scattered.direction());
                } else {
                    scattered = Ray::new(rec.p, light_pdf.generate().normalize());
                    pdf = light_pdf.value(&scattered.direction());
                }

                emitted
                + srec.attenuation * rec.mat_ptr.scattering_pdf(r, &rec, &mut scattered)
                * ray_color(&scattered, world, lights, envmap, depth + 1) / pdf
            }
        }
    } else {
        // We will hit the environment map sphere
        if let Some(rec) = envmap.hit(r, utility::NEAR_ZERO, utility::INFINITY) {
            let unit_p: Vec3A = rec.p.normalize();
            return rec.mat_ptr.emitted(rec.u, rec.v, &unit_p);
        } else { Vec3A::ONE.lerp(utility::BLUE_SKY, 0.5 * (r.direction().normalize().y + 1.0)) }
    }
}

fn get_lights(world: &HittableList) -> HittableList { world.iter().filter(|x| x.is_light()).cloned().collect() }

// Inits the scene and returns it as a HittableList
#[allow(dead_code)]
pub fn init_scene() -> HittableList {
    // Materials
    let material_ground: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let material_left: Metal = Metal::new(Color::new(0.3, 0.3, 0.3), 0.1);
    // let material_right: Metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let material_high: DiffuseLight = DiffuseLight::new(Color::new(1.0, 1.0, 1.0), 8.0);

    // World
    let mut world: HittableList = HittableList::new();
    world.push(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(material_ground), 0)));
    world.push(Arc::new(Mesh::new(Point3::new(-1.0, 1.0, 8.0), 2.5, Vec3A::new(90.0, 90.0, 220.0), "models/jet/jet2.obj", Box::new(material_left))));
    //world.push(Arc::new(Sphere::new(Point3::new(1.5, 0.5, -1.0), 0.5, Box::new(material_right), 0)));
    world.push(Arc::new(Sphere::new(Point3::new(0.0, 4.0, 0.0), 0.5, Box::new(material_high), 0)));
    _add_random_world_spheres(&mut world).expect("Failed to add random world spheres");

    let mat1: Dielectric = Dielectric::new(Vec3A::ONE, 1.5);
    world.push(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(mat1), 0)));
    let mat2: Lambertian = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(mat2), 0)));
    let mat3: Metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(mat3), 0)));

    world
}

#[allow(dead_code)]
pub fn init_random_scene() -> HittableList {
    let mut world: HittableList = HittableList::new();
    _add_random_world_spheres(&mut world).expect("Failed to add random world spheres");

    let mat1: Dielectric = Dielectric::new(Vec3A::ONE, 1.5);
    world.push(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(mat1), 0)));
    let mat2: Lambertian = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.push(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(mat2), 0)));
    let mat3: Metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(mat3), 0)));

    let ground_material: Lambertian = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Box::new(ground_material), 0)));

    world
}

fn _add_random_world_spheres(world: &mut HittableList) -> Result<(), std::io::Error> {
    let mut spheres = Vec::<Sphere>::new();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = utility::random_f32();
            let center: Point3 = Vec3A::new(a as f32 + 0.9 * utility::random_f32(), 0.2, b as f32 + 0.9 * utility::random_f32());
            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    // Lambertian
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
                    let sphere_material: Dielectric = Dielectric::new(Vec3A::ONE, 1.5);
                    spheres.push(Sphere::new(center, 0.2, Box::new(sphere_material), 0));
                }
            }
        }
    }
    let spheres_arr: SphereArray = SphereArray::new(&mut spheres);
    world.push(Arc::new(spheres_arr));
    Ok(())
}

pub fn init_scene_from_yaml(filename: &str) -> HittableList { parser::parse_yaml_scene(filename) }
