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
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Box<dyn Material>,
}

unsafe impl Sync for Sphere {}
unsafe impl Send for Sphere {}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Point3, radius: f32, material: Box<dyn Material>) -> Sphere { Sphere { center, radius, material } }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3A = ray.origin() - self.center;
        let a: f32 = ray.direction().dot(ray.direction());
        let half_b: f32 = oc.dot(ray.direction());
        let c: f32 = oc.dot(oc) - self.radius.powi(2);
        let discriminant: f32 = (half_b * half_b) - (a * c);
        if discriminant < 0.0 { return false; } // No real roots, so no intersection.
        let sqrtd: f32 = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root: f32 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root; // The ray hits the sphere at this value for 't'.
        rec.p = ray.at(rec.t); // The ray hits the sphere at this point 'p'.
        // The outward normal is the vector from the center of the sphere to the point of intersection.
        let outward_normal = (rec.p - self.center) / self.radius;
        // This function is used to determine whether the ray is inside or outside the object.
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = self.material.clone();
        true
    }
}
