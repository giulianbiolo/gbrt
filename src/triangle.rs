// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Triangle struct

use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::Point3 as BVHPoint3;

use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;
use crate::utility::EPSILON;


#[derive(Clone)]
pub struct Triangle {
    vertices: [Point3; 3],
    material: Box<dyn Material>,
    node_index: usize,
}

unsafe impl Sync for Triangle {}
unsafe impl Send for Triangle {}

impl Triangle {
    #[allow(dead_code)]
    pub fn new(vertices: [Point3; 3], material: Box<dyn Material>, node_index: usize) -> Triangle { Triangle { vertices, material, node_index } }
}

impl Bounded for Triangle {
    fn aabb(&self) -> AABB {
        let mut min: BVHPoint3 = BVHPoint3::new(self.vertices[0].x, self.vertices[0].y, self.vertices[0].z);
        let mut max: BVHPoint3 = BVHPoint3::new(self.vertices[0].x, self.vertices[0].y, self.vertices[0].z);
        for i in 1..3 {
            let vert: BVHPoint3 = BVHPoint3::new(self.vertices[i].x, self.vertices[i].y, self.vertices[i].z);
            min = min.min(vert);
            max = max.max(vert);
        }
        AABB::with_bounds(min, max)
    }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // This is a simple implementation of the Moller-Trumbore algorithm for ray-triangle intersection
        let v0 = self.vertices[0];
        let v1 = self.vertices[1];
        let v2 = self.vertices[2];

        let e1 = v1 - v0;
        let e2 = v2 - v0;

        let h = ray.direction.cross(e2);
        let a = e1.dot(h);

        if a > -EPSILON && a < EPSILON { return None; }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 { return None; }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 { return None; }

        let t = f * e2.dot(q);

        if t > t_min && t < t_max {
            let mut rec: HitRecord = HitRecord::empty();
            rec.t = t;
            rec.p = ray.at(t);
            let outward_normal: Vec3A = e2.cross(e1).normalize();
            rec.set_face_normal(ray, &outward_normal);
            rec.mat_ptr = self.material.clone();
            Some(rec)
        } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::Lambertian;

    #[test]
    fn test_triangle_hit() -> Result<(), std::fmt::Error> {
        let vertices: [Point3; 3] = [Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0), Point3::new(0.0, 1.0, 0.0)];
        let material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let triangle: Triangle = Triangle::new(vertices, material, 0);
        let ray: Ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vec3A::new(0.0, 0.0, 1.0));
        assert!(triangle.hit(&ray, 0.0, 100.0).is_some());
        Ok(())
    }
}
