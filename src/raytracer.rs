// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various raytracing functions

use std::io::Write;
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
// use crate::pdf::{PDF, HittablePDF};


#[allow(dead_code)]
pub fn calculate_phase_and_power(path: &Vec<Vec3A>) -> (f32, f32) {
    let dist: f32 = path.into_iter().zip(path.iter().skip(1)).map(|(a, b)| (*b - *a).length()).sum();
    let mut phase: f32 = path.len() as f32 * std::f32::consts::PI;
    phase += 2.0 * std::f32::consts::PI * dist / utility::CONSTS.sources_lambda; // lambda = c / freq -> lambda = c / 2.45GHz = 0.1223642686m
    phase %= 2.0 * std::f32::consts::PI; // phase is in [0, 2pi]
    // compute the power also dependent on distance and phase
    let power: f32 = 1.0 / (4.0 * std::f32::consts::PI * dist * dist);
    (phase, power)
}

// Renders the scene to an image
#[allow(dead_code)]
pub fn render_to_image(world: &HittableList, cam: &Camera, filename: &str) {
    // Render function
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(CONSTS.width, CONSTS.height);
    let envmap: Arc<dyn Hittable + Send + Sync> = load_environment();
    let mut lights: Vec<Arc<dyn Hittable + Send + Sync>> = get_lights(world);
    if CONSTS.environment_intensity.unwrap_or(1.0) > 0.0 { lights.push(envmap.clone()); }
    let filter: Box<dyn Filter + Send + Sync> = load_filter();
    println!("Lights: {}", lights.len());
    println!("Chosen Filter: {}", filter);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..CONSTS.samples_per_pixel {
            let u: f32 = (x as f32 + filter.sample(random_f32())) / (CONSTS.width as f32 - 1.0);
            let v: f32 = (CONSTS.height as f32 - (y as f32 + filter.sample(random_f32()))) as f32 / (CONSTS.height as f32 - 1.0);
            let r: Ray = cam.get_ray(u, v);
            let mut path: Vec<Vec3A> = vec![r.origin()];
            let curr_color: Vec3A = ray_color(&r, world, &lights, &envmap, 0, &mut path);
            if curr_color.is_finite() { pixel_color += curr_color; }
        }
        *pixel = to_rgb(pixel_color, CONSTS.samples_per_pixel as f32);
    }
    // Save the image
    img.save(filename).unwrap();
}

#[allow(dead_code)]
pub fn render_to_image_multithreaded(world: &HittableList, cam: Camera, filename: &str) {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(CONSTS.width, CONSTS.height);
    let safe_img: Arc<Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>>> = Arc::new(Mutex::new(img));
    let environment_map: Arc<dyn Hittable + Send + Sync> = load_environment();
    let safe_world: Arc<Vec<Arc<dyn Hittable + Send + Sync>>> = Arc::new(world.clone());
    let mut lights: Vec<Arc<dyn Hittable + Send + Sync>> = get_lights(&world);
    if CONSTS.environment_intensity.unwrap_or(1.0) > 0.0 { lights.push(environment_map.clone()); }
    let filter: Box<dyn Filter + Send + Sync> = load_filter();
    println!("Lights: {}", lights.len());
    println!("Chosen Filter: {}", filter);
    let total_rows: f32 = CONSTS.height as f32;
    let completed_rows: AtomicU32 = AtomicU32::new(0);
    (0..CONSTS.height).into_par_iter().for_each(|y| {
        for x in 0..CONSTS.width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..CONSTS.samples_per_pixel {
                let u: f32 = (x as f32 + filter.sample(random_f32())) / (CONSTS.width as f32 - 1.0);
                let v: f32 = (CONSTS.height as f32 - (y as f32 + filter.sample(random_f32()))) / (CONSTS.height as f32 - 1.0);
                let r: Ray = cam.get_ray(u, v);
                let mut path: Vec<Vec3A> = vec![r.origin()];
                let curr_color: Color = ray_color(&r, &*safe_world, &lights, &environment_map, 0, &mut path);
                if curr_color.is_finite() { pixel_color += curr_color; }
            }
            let rgb: Rgb<u8> = to_rgb(pixel_color, CONSTS.samples_per_pixel as f32);
            let mut img: std::sync::MutexGuard<'_, ImageBuffer<Rgb<u8>, Vec<u8>>> = safe_img.lock().unwrap();
            img.put_pixel(x, y as u32, rgb);
        }
        completed_rows.fetch_add(1, Ordering::Relaxed);
        print!("{:.2}% complete\r", completed_rows.load(Ordering::Relaxed) as f32 / total_rows * 100.0);
    });
    // Save the image
    safe_img.lock().unwrap().save(filename).unwrap();
}

#[allow(dead_code)]
pub fn render_power_grid(world: &HittableList, _: Camera, _: &str) {
    let environment_map: Arc<dyn Hittable + Send + Sync> = load_environment();
    let safe_world: Arc<Vec<Arc<dyn Hittable + Send + Sync>>> = Arc::new(world.clone());
    let mut lights: Vec<Arc<dyn Hittable + Send + Sync>> = get_lights(&world);
    if CONSTS.environment_intensity.unwrap_or(1.0) > 0.0 { lights.push(environment_map.clone()); }
    let filter: Box<dyn Filter + Send + Sync> = load_filter();
    println!("Lights: {}", lights.len());
    println!("Chosen Filter: {}", filter);
    // The power grid to be calculated will be a square of size 50x50 datapoints.
    let pgsx: usize = 150; // Power Grid Size X [ Must be even and < 2^32 ]
    let pgsy: usize = 10; // Power Grid Size Y [ Must be even and < 2^32 ]
    let pgsz: usize = 50; // Power Grid Size Z [ Must be even and < 2^32 ]
    let pgsx2: isize = pgsx as isize / 2; // Power Grid Size X / 2
    let pgsy2: isize = pgsy as isize / 2; // Power Grid Size Y / 2
    let pgsz2: isize = pgsz as isize / 2; // Power Grid Size Z / 2
    let pgts: f32 = 0.2;  // Power Grid Tile Size
    let total_rows: f32 = (pgsz + 1) as f32; // Total Rows
    let completed_rows: AtomicU32 = AtomicU32::new(0);

    let power_grid_mtx: Mutex<Vec<Vec<Vec<f32>>>> = Mutex::new(vec![vec![vec![0.0; pgsx + 1]; pgsy + 1]; pgsz + 1]);
    (-pgsz2..=pgsz2).into_par_iter().for_each(|o|{
        let mut powergridplane: Vec<Vec<f32>> = vec![vec![0.0; pgsx + 1]; pgsy + 1];
        for p in -pgsy2..=pgsy2 {
            // let mut powergridrow: Vec<f32> = vec![0.0; power_grid_size + 1];
            for r in -pgsx2..=pgsx2 {
                let offset_position: Vec3A = Vec3A::new(pgts * r as f32, pgts * p as f32, pgts * o as f32);
                let camera_pos: Vec3A = CONSTS.power_render_center + offset_position;
                let mut totpow: f32 = 0.0;
                for _y in 0..(CONSTS.samples_per_pixel * 512) {
                    let current_ray: Ray = Ray::new(camera_pos, utility::random_in_unit_sphere_uniform_distribution());
                    let mut path: Vec<Vec3A> = vec![current_ray.origin()];
                    let curr_color: Color = ray_color(&current_ray, &*safe_world, &lights, &environment_map, 0, &mut path);
                    // we weight the curr_color by the power of the ray, which is 1 / (4 * pi * r^2), and the sign depends on the phase (either constructive or destructive)
                    let (phase, power) = calculate_phase_and_power(&path);
                    // println!("Phase: {}, Power: {}", phase, power);
                    let curr_color: Color = curr_color * power * -phase.sin();
                    if curr_color.is_finite() { totpow += curr_color.length_squared(); }
                }
                // if totpow <= utility::NEAR_ZERO { totpow = utility::NEAR_ZERO; }
                powergridplane[(p + pgsy2) as usize][(r + pgsx2) as usize] = totpow;
                // powergridrow[(p + power_grid_size_2) as usize] = totpow;
            }
        }
        let mut powergridhold: std::sync::MutexGuard<'_, Vec<Vec<Vec<f32>>>> = power_grid_mtx.lock().unwrap();
        powergridhold[(o + pgsz2) as usize] = powergridplane;
        completed_rows.fetch_add(1, Ordering::Relaxed);
        print!("{:.2}% complete\r", completed_rows.load(Ordering::Relaxed) as f32 / total_rows * 100.0);
        let _ = std::io::stdout().flush();
    });
    println!("Now filtering with median 3x3 kernel...");
    let mut power_grid: Vec<Vec<Vec<f32>>> = power_grid_mtx.lock().unwrap().to_vec();
    // apply a 2d median filter on the power grid matrix
    let mut power_grid_filtered: Vec<Vec<Vec<f32>>> = vec![vec![vec![0.0; pgsx + 1]; pgsy + 1]; pgsz + 1];
    for o in 0..=pgsz {
        for p in 0..=pgsy {
            for r in 0..=pgsx {
                let mut values: Vec<f32> = Vec::new();
                for k in -1..=1 {
                    for i in -1..=1 {
                        for j in -1..=1 {
                            if o as isize + k >= 0 && o as isize + k <= pgsz as isize
                            && p as isize + j >= 0 && p as isize + j <= pgsy as isize
                            && r as isize + i >= 0 && r as isize + i <= pgsx as isize {
                                values.push(power_grid[(o as isize + k) as usize][(p as isize + j) as usize][(r as isize + i) as usize]);
                            }
                        }
                    }
                }
                values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                power_grid_filtered[o][p][r] = values[values.len() / 2];
            }
        }
    }
    power_grid = power_grid_filtered;

    println!("Now building the decibels representation...");
    let mut power_grid_decibel: Vec<Vec<Vec<f32>>> = vec![vec![vec![0.0; pgsx + 1]; pgsy + 1]; pgsz + 1];
    // let us create an image and save the power grid in it as values of gray
    //first of all we normalize the values in the grid between 0 and 1
    let mut max: f32 = 0.0;
    for o in 0..=pgsz {
        for p in 0..=pgsy {
            for r in 0..=pgsx {
                if power_grid[o][p][r] > max { max = power_grid[o][p][r]; }
            }
        }
    }
    for o in 0..=pgsz {
        for p in 0..=pgsy {
            for r in 0..=pgsx {
                // assuming the max is the source power, rewrite values as decibels
                power_grid[o][p][r] /= max; // now they are all between 0 and 1
                power_grid_decibel[o][p][r] = 10.0 * (power_grid[o][p][r]).log10(); // now they are all in decibels and negative!
            }
        }
    }
    println!("Now saving the test images...");
    // now we create the image
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(pgsx as u32 + 1, pgsy as u32 + 1);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut rgb: Rgb<u8> = Rgb::from([0, 0, 0]);
        // the y must be inverted because the image is saved upside down
        let color: u8 = (power_grid[0][(pgsy - y as usize) as usize][x as usize] * 255.0) as u8;
        rgb[0] = color;
        rgb[1] = color;
        rgb[2] = color;
        *pixel = rgb;
    }
    // Save the image
    img.save("power_grid.png").unwrap();
    // resize the image to 8x
    let nimg = image::open("power_grid.png").unwrap();
    let gaussian = nimg.resize(800, 800, image::imageops::FilterType::Gaussian);
    gaussian.save("power_grid_gau.png").unwrap();
    // save the power grid values of decibels to a byte file
    println!("Now saving the data files...");
    let mut file = std::fs::File::create("power_grid_db.bin").unwrap();
    file.write(&(pgsx + 1).to_le_bytes()).unwrap();
    file.write(&(pgsy + 1).to_le_bytes()).unwrap();
    file.write(&(pgsz + 1).to_le_bytes()).unwrap();
    for o in 0..=pgsz {
        for p in 0..=pgsy {
            for r in 0..=pgsx {
                file.write_all(&power_grid_decibel[o][p][r].to_le_bytes()).unwrap();
            }
        }
    }
    let mut file = std::fs::File::create("power_grid_lin.bin").unwrap();
    file.write(&(pgsx + 1).to_le_bytes()).unwrap();
    file.write(&(pgsy + 1).to_le_bytes()).unwrap();
    file.write(&(pgsz + 1).to_le_bytes()).unwrap();
    for o in 0..=pgsz {
        for p in 0..=pgsy {
            for r in 0..=pgsx {
                file.write_all(&power_grid[o][p][r].to_le_bytes()).unwrap();
            }
        }
    }
}

// Returns the color of a ray
pub fn ray_color(r: &Ray, world: &HittableList, lights: &HittableList, envmap: &Arc<dyn Hittable + Sync + Send>, depth: u32, path: &mut Vec<Vec3A>) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if unlikely(depth >= CONSTS.max_depth) {
        path.push(r.origin()); // not sure if needed
        return Color::new(0.0, 0.0, 0.0);
    }
    // Check for ray-object intersection
    if let Some(rec) = world.hit(r, utility::NEAR_ZERO, utility::INFINITY) {
        let emitted: Vec3A = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p);
        // If the material is light, return the emittance
        if rec.mat_ptr.is_light() {
            path.push(rec.p);
            return emitted; // color * intensity(power)
        }
        // If the material is not light, we first need to scatter the ray
        let mut srec: ScatterRecord = ScatterRecord::new();
        // If the ray doesn't scatter, we return the emittance of the object, not scattering means the ray is absorbed by the object
        if !rec.mat_ptr.scatter(r, &rec, &mut srec) {
            path.push(rec.p);
            return emitted;
        }
        // We Russian Roulette some of the rays that are old enough
        if depth > utility::CONSTS.min_depth && utility::random_f32() < srec.attenuation.max_element() {
            path.push(rec.p);
            return emitted;
        }
        // If the material is specular, we can just return the color of the specular ray
        if srec.is_specular {
            path.push(srec.specular_ray.origin());
            return srec.attenuation * ray_color(&srec.specular_ray, world, lights, envmap, depth + 1, path);
        }
        // We are now in the realm of diffuse materials, we work with PDFs
        // Not using the PDF classes to improve performance, altough those classes are implemented in the pdf.rs file for reference
        let mut scattered: Ray = Ray::new(rec.p, if utility::random_f32() < 0.5 {srec.pdf_ptr.clone().unwrap().generate()} else {lights.random(&rec.p).normalize()});
        let pdf: f32 = 0.5 * srec.pdf_ptr.unwrap().value(&scattered.direction()) + 0.5 * lights.pdf_value(&rec.p, &scattered.direction());
        // Finally, we return the color of the scattered ray
        path.push(scattered.origin());
        return emitted
        + srec.attenuation * rec.mat_ptr.scattering_pdf(r, &rec, &mut scattered)
        * ray_color(&scattered, world, lights, envmap, depth + 1, path) / pdf;
    } else {
        if let Some(rec) = envmap.hit(r, utility::NEAR_ZERO, utility::INFINITY) {
            path.push(rec.p);
            rec.mat_ptr.emitted(rec.u, rec.v, &rec.p)
        } else {
            path.push(r.origin());
            Vec3A::ONE.lerp(utility::BLUE_SKY, 0.5 * (r.direction().normalize().y + 1.0))
        }
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

    let mat1: Dielectric = Dielectric::new(Vec3A::ONE, 1.5, 0.0);
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

    let mat1: Dielectric = Dielectric::new(Vec3A::ONE, 1.5, 0.0);
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
                    let sphere_material: Dielectric = Dielectric::new(Vec3A::ONE, 1.5, 0.0);
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
