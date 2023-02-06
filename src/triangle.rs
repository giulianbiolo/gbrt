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
            if self.vertices[i].x < min.x { min.x = self.vertices[i].x; }
            if self.vertices[i].y < min.y { min.y = self.vertices[i].y; }
            if self.vertices[i].z < min.z { min.z = self.vertices[i].z; }
            if self.vertices[i].x > max.x { max.x = self.vertices[i].x; }
            if self.vertices[i].y > max.y { max.y = self.vertices[i].y; }
            if self.vertices[i].z > max.z { max.z = self.vertices[i].z; }
        }
        AABB::with_bounds(min, max)
    }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        // This is a simple implementation of the Moller-Trumbore algorithm for ray-triangle intersection
        let v0 = self.vertices[0];
        let v1 = self.vertices[1];
        let v2 = self.vertices[2];

        let e1 = v1 - v0;
        let e2 = v2 - v0;

        let h = ray.direction.cross(e2);
        let a = e1.dot(h);

        if a > -EPSILON && a < EPSILON { return false; }

        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 { return false; }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 { return false; }

        let t = f * e2.dot(q);

        if t > t_min && t < t_max {
            rec.t = t;
            rec.p = ray.at(t);
            let outward_normal: Vec3A = e2.cross(e1).normalize();
            rec.set_face_normal(ray, &outward_normal);
            rec.mat_ptr = self.material.clone();
            return true;
        }
        false
    }
}
