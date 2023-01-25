// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the main function of the project

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
mod raytracer;

use hittable_list::HittableList;
use camera::Camera;
use vec3::Vec3;



fn main() -> Result<(), std::fmt::Error> {
    // Print various logs
    println!("Image Size: {}x{}", utility::WIDTH, utility::HEIGHT);
    // Initialize the scene
    // let world: HittableList = raytracer::init_scene();
    let world: HittableList = raytracer::init_random_scene();
    // Initialize the camera
    let cam: Camera = Camera::new(
        &Vec3::new(13.0, 2.0, 3.0),
        &Vec3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        40.0,
        utility::ASPECT_RATIO,
        0.1,
        10.0,
    );
    // Render the scene to an image
    raytracer::render_to_image(&world, &cam, "test.png");
    Ok(())
}
