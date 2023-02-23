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
use crate::utility::{NEAR_ZERO, EPSILON, self};


#[derive(Clone)]
pub struct Triangle {
    vertices: Box<[Point3; 3]>,
    normals: Box<[Vec3A; 3]>,
    material: Box<dyn Material>,
    node_index: usize,
}

unsafe impl Sync for Triangle {}
unsafe impl Send for Triangle {}

impl Triangle {
    #[allow(dead_code)]
    pub fn new(mut vertices: Box<[Point3; 3]>, mut normals: Box<[Vec3A; 3]>, material: Box<dyn Material>, node_index: usize) -> Triangle {
        // We repair the normals if they are not pointing in the right direction
        _check_repair_normals(&mut vertices, &mut normals);
        Triangle { vertices, normals, material, node_index }
    }
    // fn _get_triangle_uv(&self, p: &Vec3A) -> (f32, f32) { ((p.x - self.vertices[0].x) / (self.vertices[1].x - self.vertices[0].x), (p.y - self.vertices[0].y) / (self.vertices[2].y - self.vertices[0].y)) }
    fn _get_triangle_uv(&self, p: &Vec3A) -> (f32, f32) {
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let q = *p - self.vertices[0];
        let denominator = e1.x * e2.y - e1.y * e2.x;
        let u = (q.x * e2.y - q.y * e2.x) / denominator;
        let v = (q.y * e1.x - q.x * e1.y) / denominator;
        (u, v)
    }
    fn _get_triangle_normal(&self, u: f32, v: f32) -> Vec3A { self.normals[0] * (1.0 - u - v) + self.normals[1] * u + self.normals[2] * v }
    pub fn check_not_degenerate(&self) -> bool {
        (self.vertices[0] - self.vertices[1]).length() > NEAR_ZERO &&
        (self.vertices[1] - self.vertices[2]).length() > NEAR_ZERO &&
        (self.vertices[2] - self.vertices[0]).length() > NEAR_ZERO
    }
}

fn _check_repair_normals(vertices: &mut Box<[Point3; 3]>, normals: &mut Box<[Vec3A; 3]>) {
    let n = (vertices[1] - vertices[0]).cross(vertices[2] - vertices[0]).normalize();
    if normals[0].length_squared() < NEAR_ZERO { normals[0] = n; }
    if normals[1].length_squared() < NEAR_ZERO { normals[1] = n; }
    if normals[2].length_squared() < NEAR_ZERO { normals[2] = n; }
    _fix_winding_order(vertices, normals);
}
fn _fix_winding_order(vertices: &mut Box<[Point3; 3]>, normals: &mut Box<[Vec3A; 3]>) {
    let n = (vertices[1] - vertices[0]).cross(vertices[2] - vertices[0]).normalize();
    let all_normals_have_wrong_orientation = normals[0].dot(n) < 0.0 && normals[1].dot(n) < 0.0 && normals[2].dot(n) < 0.0;
    if all_normals_have_wrong_orientation {
        // Swap vertices 1 and 2 and normals 1 and 2
        let temp_vertex: Vec3A = vertices[1];
        vertices[1] = vertices[2];
        vertices[2] = temp_vertex;
        let temp_normal: Vec3A = normals[1];
        normals[1] = normals[2];
        normals[2] = temp_normal;
    }
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
        let v0: Vec3A = self.vertices[0];

        let e1: Vec3A = self.vertices[1] - v0;
        let e2: Vec3A = self.vertices[2] - v0;

        let p: Vec3A = ray.direction.cross(e2);
        let det: f32 = e1.dot(p);

        if det.abs() < EPSILON { return None; }

        let inv_det: f32 = 1.0 / det;
        let s: Vec3A = ray.origin - v0;
        let u: f32 = inv_det * s.dot(p);

        if u < 0.0 || u > 1.0 { return None; }

        let q: Vec3A = s.cross(e1);
        let v: f32 = inv_det * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 { return None; }

        let t: f32 = inv_det * e2.dot(q);

        if t > t_min && t < t_max {
            // let (u, v) = self._get_triangle_uv(&ray.at(t));
            let mut rec: HitRecord = HitRecord::new(
                ray.at(t),
                self._get_triangle_normal(u, v).normalize(),
                self.material.clone(),
                t,
                u,
                v,
                false
            );
            rec.set_face_normal(ray, &rec.normal.clone());
            Some(rec)
        } else { None }
    }
    fn is_light(&self) -> bool { self.material.is_light() }
    fn pdf_value(&self, origin: &Point3, _v: &Vec3A) -> f32 {
        let area: f32 = (self.vertices[1] - self.vertices[0]).cross(self.vertices[2] - self.vertices[0]).length() / 2.0;
        let distance_squared: f32 = (self.vertices[0] - *origin).length_squared();
        let cosine: f32 = (self.vertices[0] - *origin).dot(self._get_triangle_normal(0.0, 0.0).normalize()) / (self.vertices[0] - *origin).length();
        distance_squared / (cosine * area)
    }
    fn random(&self, origin: &Point3) -> Vec3A {
        let u: f32 = utility::random_f32();
        let v: f32 = utility::random_f32();
        let w: f32 = 1.0 - u - v;
        (self.vertices[0] * w + self.vertices[1] * u + self.vertices[2] * v) - *origin
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
        let normals = Box::new([Vec3A::new(0.0, 0.0, 1.0), Vec3A::new(0.0, 0.0, 1.0), Vec3A::new(0.0, 0.0, 1.0)]);
        let triangle: Triangle = Triangle::new(Box::new(vertices), normals, material, 0);
        let ray: Ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vec3A::new(0.0, 0.0, 1.0));
        assert!(triangle.hit(&ray, 0.0, 100.0).is_some());
        Ok(())
    }
}
