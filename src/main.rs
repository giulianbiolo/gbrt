// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the main function of the project
mod point3;
mod color;
mod ray;
mod hit_record;
mod sphere;
mod hittable_list;
mod utility;
mod camera;
mod material;
mod bbox;
mod triangle;
mod obj_mesh;
mod raytracer;

use glam::Vec3A;

use hittable_list::HittableList;
use camera::Camera;



fn main() -> Result<(), std::fmt::Error> {
    let start_time = std::time::Instant::now();
    // Print various logs
    println!("Image Size: {}x{}", utility::WIDTH, utility::HEIGHT);
    // Initialize the scene
    //let world: HittableList<Sphere> = raytracer::init_random_scene();
    // Initialize the camera
    /*let cam: Camera = Camera::new(
        &Vec3A::new(13.0, 2.0, 3.0),
        &Vec3A::new(0.0, 0.0, 0.0),
        &Vec3A::new(0.0, 1.0, 0.0),
        40.0,
        utility::ASPECT_RATIO,
        0.1,
        10.0,
    );*/
    let world: HittableList = raytracer::init_scene();
    let cam: Camera = Camera::new(
        &Vec3A::new(0.0, 0.0, 0.5),
        &Vec3A::new(0.0, 0.0, -1.0),
        &Vec3A::new(0.0, 1.0, 0.0),
        90.0,
        utility::ASPECT_RATIO,
        0.0,
        10.0,
    );
    // Render the scene to an image
    // raytracer::render_to_image(&world, &cam, "test.png");
    raytracer::render_to_image_multithreaded(&world, cam, "test.png");
    let end_time = std::time::Instant::now();
    println!("Elapsed time: {}ms", end_time.duration_since(start_time).as_millis());
    Ok(())
}
