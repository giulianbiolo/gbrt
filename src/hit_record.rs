// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Hittable trait and the HitRecord struct
use glam;
use glam::Vec3A;

use crate::ray::Ray;
use crate::material::{Material, Lambertian};
use crate::color::Color;
use crate::point3::Point3;



#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3A,
    pub mat_ptr: Box<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3A::new(0.0, 0.0, 0.0),
            mat_ptr: Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            front_face: false
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3A) {
        // This function is used to determine whether the ray is inside or outside the object.
        self.front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}



// Unit Tests are to be found in the HittableList struct in src\hittable_list.rs
