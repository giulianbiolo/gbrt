// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Rectangle struct

use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::Point3 as BVHPoint3;

use glam;
use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::translate::Translatable;
use crate::utility;


/*********************** XY Rectangle ***********************/
#[derive(Clone)]
pub struct XYRectangle {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Box<dyn Material>,
    node_index: usize,
}

unsafe impl Sync for XYRectangle {}
unsafe impl Send for XYRectangle {}

impl Translatable for XYRectangle {}

impl XYRectangle {
    #[allow(dead_code)]
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Box<dyn Material>, node_index: usize) -> XYRectangle { XYRectangle { x0, x1, y0, y1, k, material, node_index } }
}

impl Bounded for XYRectangle {
    fn aabb(&self) -> AABB {
        let min: BVHPoint3 = BVHPoint3::new(self.x0, self.y0, self.k - 0.0001);
        let max: BVHPoint3 = BVHPoint3::new(self.x1, self.y1, self.k + 0.0001);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for XYRectangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for XYRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t: f32 = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max { return false; }
        let x: f32 = ray.origin().x + t * ray.direction().x;
        let y: f32 = ray.origin().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 { return false; }
        if y < self.y0 || y > self.y1 { return false; }
        //rec.u = (x - self.x0) / (self.x1 - self.x0);
        //rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        rec.p = ray.at(t);
        let outward_normal: Vec3A = Vec3A::new(0.0, 0.0, 1.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = self.material.clone();
        true
    }
    fn is_light(&self) -> bool { self.material.is_light() }
    fn pdf_value(&self, origin: &Vec3A, v: &Vec3A) -> f32 {
        let mut rec: HitRecord = HitRecord::empty();
        if self.hit(&Ray::new(*origin, *v), 0.001, utility::INFINITY, &mut rec) {
            let area: f32 = (self.x1 - self.x0) * (self.y1 - self.y0);
            let distance_squared: f32 = rec.t * rec.t * v.length_squared();
            let cosine: f32 = (v.dot(rec.normal) / v.length()).abs();
            distance_squared / (cosine * area)
        } else { 0.0 }
    }
    fn random(&self, origin: &Vec3A) -> Vec3A {
        let random_point: Vec3A = Vec3A::new(
            utility::random_f32_range(self.x0, self.x1),
            utility::random_f32_range(self.y0, self.y1),
            self.k,
        );
        random_point - *origin
    }
}


/*********************** XZ Rectangle ***********************/
#[derive(Clone)]
pub struct XZRectangle {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Box<dyn Material>,
    node_index: usize,
}

unsafe impl Sync for XZRectangle {}
unsafe impl Send for XZRectangle {}

impl Translatable for XZRectangle {}

impl XZRectangle {
    #[allow(dead_code)]
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Box<dyn Material>, node_index: usize) -> XZRectangle { XZRectangle { x0, x1, z0, z1, k, material, node_index } }
}

impl Bounded for XZRectangle {
    fn aabb(&self) -> AABB {
        let min: BVHPoint3 = BVHPoint3::new(self.x0, self.k - 0.0001, self.z0);
        let max: BVHPoint3 = BVHPoint3::new(self.x1, self.k + 0.0001, self.z1);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for XZRectangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for XZRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t: f32 = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max { return false; }
        let x: f32 = ray.origin().x + t * ray.direction().x;
        let z: f32 = ray.origin().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 { return false; }
        if z < self.z0 || z > self.z1 { return false; }
        //rec.u = (x - self.x0) / (self.x1 - self.x0);
        //rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.p = ray.at(t);
        let outward_normal: Vec3A = Vec3A::new(0.0, 1.0, 0.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = self.material.clone();
        true
    }
    fn pdf_value(&self, origin: &Vec3A, v: &Vec3A) -> f32 {
        let mut rec: HitRecord = HitRecord::empty();
        if !self.hit(&Ray::new(*origin, *v), 0.001, f32::INFINITY, &mut rec) { return 0.0; }
        let area: f32 = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared: f32 = rec.t * rec.t * v.length_squared();
        let cosine: f32 = v.dot(rec.normal).abs() / v.length();
        distance_squared / (cosine * area)
    }
    fn random(&self, origin: &Vec3A) -> Vec3A {
        let random_point: Vec3A = Vec3A::new(
            utility::random_f32_range(self.x0, self.x1),
            self.k,
            utility::random_f32_range(self.z0, self.z1)
        );
        random_point - *origin
    }
    fn is_light(&self) -> bool { self.material.is_light() }
}


/*********************** YZ Rectangle ***********************/
#[derive(Clone)]
pub struct YZRectangle {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Box<dyn Material>,
    node_index: usize,
}

unsafe impl Sync for YZRectangle {}
unsafe impl Send for YZRectangle {}

impl Translatable for YZRectangle {}

impl YZRectangle {
    #[allow(dead_code)]
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Box<dyn Material>, node_index: usize) -> YZRectangle { YZRectangle { y0, y1, z0, z1, k, material, node_index } }
}

impl Bounded for YZRectangle {
    fn aabb(&self) -> AABB {
        let min: BVHPoint3 = BVHPoint3::new(self.k - 0.0001, self.y0, self.z0);
        let max: BVHPoint3 = BVHPoint3::new(self.k + 0.0001, self.y1, self.z1);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for YZRectangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for YZRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let t: f32 = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max { return false; }
        let y: f32 = ray.origin().y + t * ray.direction().y;
        let z: f32 = ray.origin().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 { return false; }
        if z < self.z0 || z > self.z1 { return false; }
        //rec.u = (y - self.y0) / (self.y1 - self.y0);
        //rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        rec.p = ray.at(t);
        let outward_normal: Vec3A = Vec3A::new(1.0, 0.0, 0.0);
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = self.material.clone();
        true
    }
    fn is_light(&self) -> bool { self.material.is_light() }
    fn pdf_value(&self, origin: &Vec3A, v: &Vec3A) -> f32 {
        let mut rec: HitRecord = HitRecord::empty();
        if !self.hit(&Ray::new(*origin, *v), 0.001, f32::INFINITY, &mut rec) { return 0.0; }
        let area: f32 = (self.y1 - self.y0) * (self.z1 - self.z0);
        let distance_squared: f32 = rec.t * rec.t * v.length_squared();
        let cosine: f32 = v.dot(rec.normal).abs() / v.length();
        distance_squared / (cosine * area)
    }
    fn random(&self, origin: &Vec3A) -> Vec3A {
        let random_point: Vec3A = Vec3A::new(
            self.k,
            utility::random_f32_range(self.y0, self.y1),
            utility::random_f32_range(self.z0, self.z1)
        );
        random_point - *origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::Lambertian;
    use crate::point3::Point3;

    #[test]
    fn test_xyrectangle_hit() {
        let material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let rectangle: XYRectangle = XYRectangle::new(-1.0, 1.0, -1.0, 1.0, 0.0, material, 0);
        let ray: Ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vec3A::new(0.0, 0.0, 1.0));
        let mut rec: HitRecord = HitRecord::empty();
        assert!(rectangle.hit(&ray, 0.0, 100.0, &mut rec));
    }
    #[test]
    fn test_xzrectangle_hit() {
        let material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let rectangle: XZRectangle = XZRectangle::new(-1.0, 1.0, -1.0, 1.0, 0.0, material, 0);
        let ray: Ray = Ray::new(Point3::new(0.0, -1.0, 0.0), Vec3A::new(0.0, 1.0, 0.0));
        let mut rec: HitRecord = HitRecord::empty();
        assert!(rectangle.hit(&ray, 0.0, 100.0, &mut rec));
    }
    #[test]
    fn test_yzrectangle_hit() {
        let material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let rectangle: YZRectangle = YZRectangle::new(-1.0, 1.0, -1.0, 1.0, 0.0, material, 0);
        let ray: Ray = Ray::new(Point3::new(-1.0, 0.0, 0.0), Vec3A::new(1.0, 0.0, 0.0));
        let mut rec: HitRecord = HitRecord::empty();
        assert!(rectangle.hit(&ray, 0.0, 100.0, &mut rec));
    }
}
