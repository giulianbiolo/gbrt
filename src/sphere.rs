// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Sphere struct

use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::Point3 as BVHPoint3;

use glam::Vec3A;

use crate::onb::ONB;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;
use crate::utility;


#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    material: Box<dyn Material>,
    node_index: usize,
}

unsafe impl Sync for Sphere {}
unsafe impl Send for Sphere {}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Point3, radius: f32, material: Box<dyn Material>, node_index: usize) -> Sphere { Sphere { center, radius, material, node_index } }
    fn _get_sphere_uv(&self, p: &Vec3A) -> (f32, f32) {
        let unit_p: Vec3A = (*p - self.center) / self.radius;
        (((-unit_p.z).atan2(unit_p.x) + utility::PI) / (2.0 * utility::PI), ((-unit_p.y).acos()) / utility::PI)
    }
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
        let (u, v) = self._get_sphere_uv(&ray.at(root));
        let mut rec: HitRecord = HitRecord::new(
            ray.at(root),
            (ray.at(root) - self.center) / self.radius,
            self.material.clone(),
            root,
            u,
            v,
            false
        );
        rec.set_face_normal(ray, &rec.normal.clone());
        Some(rec)
    }
    fn is_light(&self) -> bool { self.material.is_light() }
    fn pdf_value(&self, origin: &Point3, _: &Vec3A) -> f32 {
        let sphere_center_to_camera: Vec3A = *origin - self.center;
        let distance_to_camera_squared: f32 = sphere_center_to_camera.length_squared();
        let cos_theta_max: f32 = ((1.0 - (self.radius * self.radius)) / distance_to_camera_squared).max(0.0);
        let solid_angle: f32 = 2.0 * utility::PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }
    fn random(&self, origin: &Point3) -> Vec3A {
        let sphere_center_to_camera: Vec3A = *origin - self.center;
        let distance_to_camera_squared: f32 = sphere_center_to_camera.length_squared();
        let cos_theta_max: f32 = (1.0 - (self.radius * self.radius) / (distance_to_camera_squared)).max(0.0);
        // let sin_theta_max: f32 = (1.0 - cos_theta_max*cos_theta_max).sqrt();
        let phi: f32 = utility::random_f32_range(0.0, 2.0 * utility::PI);
        let cos_theta: f32 = utility::random_f32_range(cos_theta_max, 1.0);
        let sin_theta: f32 = (1.0 - (cos_theta * cos_theta)).sqrt();
        let direction: Vec3A = Vec3A::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta);
        let sphere_center_to_camera_unit: Vec3A = sphere_center_to_camera.normalize();
        let mut onb: ONB = ONB::new();
        onb.build_from_w(&sphere_center_to_camera_unit);
        onb.local_vec(&direction)
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
