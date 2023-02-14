// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Sphere struct

use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::Point3 as BVHPoint3;

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
    node_index: usize,
}

unsafe impl Sync for Sphere {}
unsafe impl Send for Sphere {}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Point3, radius: f32, material: Box<dyn Material>, node_index: usize) -> Sphere { Sphere { center, radius, material, node_index } }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let min: BVHPoint3 = BVHPoint3::new(self.center.x - self.radius, self.center.y - self.radius, self.center.z - self.radius);
        let max: BVHPoint3 = BVHPoint3::new(self.center.x + self.radius, self.center.y + self.radius, self.center.z + self.radius);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for Sphere {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3A = ray.origin() - self.center;
        let a: f32 = ray.direction().dot(ray.direction());
        let half_b: f32 = oc.dot(ray.direction());
        let c: f32 = oc.dot(oc) - self.radius.powi(2);
        let discriminant: f32 = (half_b * half_b) - (a * c);
        if discriminant < 0.0 { return None; } // No real roots, so no intersection.
        let sqrtd: f32 = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root: f32 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec: HitRecord = HitRecord::empty();
        rec.t = root; // The ray hits the sphere at this value for 't'.
        rec.p = ray.at(rec.t); // The ray hits the sphere at this point 'p'.
        // The outward normal is the vector from the center of the sphere to the point of intersection.
        let outward_normal: Vec3A = (rec.p - self.center) / self.radius;
        // This function is used to determine whether the ray is inside or outside the object.
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = self.material.clone();
        Some(rec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::Lambertian;

    #[test]
    fn test_sphere_hit() -> Result<(), std::fmt::Error> {
        let sphere: Sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))), 0);
        let ray: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, -1.0));
        assert!(sphere.hit(&ray, 0.0, 100.0).is_some());
        Ok(())
    }
}
