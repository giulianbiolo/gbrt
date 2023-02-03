// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Sphere struct
use glam;
use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;


#[derive(Clone)]
pub struct Rectangle {
    center: Point3,
    width: f32,
    height: f32,
    depth: f32,
    material: Box<dyn Material>,
}

unsafe impl Sync for Rectangle {}
unsafe impl Send for Rectangle {}

impl Rectangle {
    pub fn new(center: Point3, width: f32, height: f32, depth: f32, material: Box<dyn Material>) -> Rectangle { Rectangle { center, width, height, depth, material } }
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        // Check if the ray is parallel to the plane
        // Not Yet Implemented
        // ...
        true
    }
}
