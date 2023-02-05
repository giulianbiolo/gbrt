// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the BBox struct
use glam;
use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;


#[derive(Clone)]
pub struct BBox {
    center: Point3,
    width: f32,
    height: f32,
    depth: f32,
    material: Box<dyn Material>,
}

unsafe impl Sync for BBox {}
unsafe impl Send for BBox {}

impl BBox {
    #[allow(dead_code)]
    pub fn new(center: Point3, dimensions: Vec3A, material: Box<dyn Material>) -> BBox {
        BBox {
            center,
            width: dimensions[0],
            height: dimensions[1],
            depth: dimensions[2],
            material
        }
    }
}

impl Hittable for BBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        // Calculate the interval of possible hit times along the x axis
        let mut tmin = (self.center.x - self.width / 2.0 - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.center.x + self.width / 2.0 - ray.origin.x) / ray.direction.x;
        // Swap tmin and tmax if necessary
        if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }
        // Calculate the interval of possible hit times along the y axis
        let mut tymin = (self.center.y - self.height / 2.0 - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.center.y + self.height / 2.0 - ray.origin.y) / ray.direction.y;
        // Swap tymin and tymax if necessary
        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }
        // If the intervals along the x and y axis do not overlap, the ray does not hit the box
        if tmin > tymax || tymin > tmax { return false; }
        // If the intervals along the x and y axis overlap, the ray hits the box
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }
        // Calculate the interval of possible hit times along the z axis
        let mut tzmin = (self.center.z - self.depth / 2.0 - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.center.z + self.depth / 2.0 - ray.origin.z) / ray.direction.z;
        // Swap tzmin and tzmax if necessary
        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }
        // If the intervals along the x and z axis do not overlap, the ray does not hit the box
        if tmin > tzmax || tzmin > tmax { return false; }
        // If the intervals along the x and z axis overlap, the ray hits the box
        if tzmin > tmin { tmin = tzmin; }
        if tzmax < tmax { tmax = tzmax; }
        // Check if the hit time is within the allowed range
        if tmin < t_max && tmax > t_min {
            let mut t = tmin;
            if t < t_min { t = tmax; }
            if t > t_min && t < t_max {
                let p = ray.at(t);
                rec.t = t;
                rec.p = p;
                rec.normal = (p - self.center) / Vec3A::new(self.width, self.height, self.depth);
                rec.mat_ptr = self.material.clone();
                return true;
            }
        }
        false
    }
}
