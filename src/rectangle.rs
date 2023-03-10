// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the various Rectangles structs

use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::Point3 as BVHPoint3;

use glam::Vec3A;

use crate::point3::Point3;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::utility;


/*********************** Rectangle ***********************/
pub enum Rectangle {
    XZRectangle(XZRectangle),
    YZRectangle(YZRectangle),
    XYRectangle(XYRectangle),
}

impl Bounded for Rectangle {
    fn aabb(&self) -> AABB {
        match self {
            Rectangle::XZRectangle(xz_rectangle) => xz_rectangle.aabb(),
            Rectangle::YZRectangle(yz_rectangle) => yz_rectangle.aabb(),
            Rectangle::XYRectangle(xy_rectangle) => xy_rectangle.aabb(),
        }
    }
}

impl BHShape for Rectangle {
    fn set_bh_node_index(&mut self, index: usize) {
        match self {
            Rectangle::XZRectangle(xz_rectangle) => xz_rectangle.set_bh_node_index(index),
            Rectangle::YZRectangle(yz_rectangle) => yz_rectangle.set_bh_node_index(index),
            Rectangle::XYRectangle(xy_rectangle) => xy_rectangle.set_bh_node_index(index),
        }
    }
    fn bh_node_index(&self) -> usize {
        match self {
            Rectangle::XZRectangle(xz_rectangle) => xz_rectangle.bh_node_index(),
            Rectangle::YZRectangle(yz_rectangle) => yz_rectangle.bh_node_index(),
            Rectangle::XYRectangle(xy_rectangle) => xy_rectangle.bh_node_index(),
        }
    }
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Rectangle::XZRectangle(xz_rectangle) => xz_rectangle.hit(ray, t_min, t_max),
            Rectangle::YZRectangle(yz_rectangle) => yz_rectangle.hit(ray, t_min, t_max),
            Rectangle::XYRectangle(xy_rectangle) => xy_rectangle.hit(ray, t_min, t_max),
        }
    }
    fn is_light(&self) -> bool {
        match self {
            Rectangle::XZRectangle(xz_rectangle) => xz_rectangle.is_light(),
            Rectangle::YZRectangle(yz_rectangle) => yz_rectangle.is_light(),
            Rectangle::XYRectangle(xy_rectangle) => xy_rectangle.is_light(),
        }
    }
    fn pdf_value(&self, o: &crate::point3::Point3, v: &Vec3A) -> f32 {
        match self {
            Rectangle::XZRectangle(xz_rectangle) => xz_rectangle.pdf_value(o, v),
            Rectangle::YZRectangle(yz_rectangle) => yz_rectangle.pdf_value(o, v),
            Rectangle::XYRectangle(xy_rectangle) => xy_rectangle.pdf_value(o, v),
        }
    }
    fn random(&self, o: &crate::point3::Point3) -> Vec3A {
        match self {
            Rectangle::XZRectangle(xz_rectangle) => xz_rectangle.random(o),
            Rectangle::YZRectangle(yz_rectangle) => yz_rectangle.random(o),
            Rectangle::XYRectangle(xy_rectangle) => xy_rectangle.random(o),
        }
    }
}


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

impl XYRectangle {
    #[allow(dead_code)]
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Box<dyn Material>, node_index: usize) -> Self { XYRectangle { x0, x1, y0, y1, k, material, node_index } }
    fn _get_xyrect_uv(&self, p: &Vec3A) -> (f32, f32) { ((p.x - self.x0) / (self.x1 - self.x0), (p.y - self.y0) / (self.y1 - self.y0)) }
}

impl Bounded for XYRectangle {
    fn aabb(&self) -> AABB {
        let min: BVHPoint3 = BVHPoint3::new(self.x0, self.y0, self.k - utility::EPSILON);
        let max: BVHPoint3 = BVHPoint3::new(self.x1, self.y1, self.k + utility::EPSILON);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for XYRectangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for XYRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t: f32 = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max { return None; }
        let xyz: Vec3A = ray.origin() + t * ray.direction();
        if xyz.x < self.x0 || xyz.x > self.x1 || xyz.y < self.y0 || xyz.y > self.y1 { return None; }
        let (u, v) = self._get_xyrect_uv(&xyz);
        let mut rec: HitRecord = HitRecord::new(
            ray.at(t),
            Vec3A::Z,
            self.material.clone(),
            t,
            u,
            v,
            false
        );
        rec.set_face_normal(ray, &rec.normal.clone());
        Some(rec)
    }
    fn is_light(&self) -> bool { self.material.is_light() }
    fn pdf_value(&self, origin: &Point3, v: &Vec3A) -> f32 {
        let t: f32 = (self.k - origin.z) / v.z;
        if t < utility::NEAR_ZERO || t > f32::INFINITY { return 0.0; }
        let xyz: Vec3A = *origin + t * *v;
        if xyz.x < self.x0 || xyz.x > self.x1 || xyz.y < self.y0 || xyz.y > self.y1 { return 0.0; }
        let area: f32 = (self.x1 - self.x0) * (self.y1 - self.y0);
        let distance_squared: f32 = t * t * v.length_squared();
        let cosine: f32 = (v.dot(Vec3A::Z) / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, origin: &Point3) -> Vec3A {
        Point3::new(
            utility::random_f32_range(self.x0, self.x1),
            utility::random_f32_range(self.y0, self.y1),
            self.k
        ) - *origin
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

impl XZRectangle {
    #[allow(dead_code)]
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Box<dyn Material>, node_index: usize) -> Self { XZRectangle { x0, x1, z0, z1, k, material, node_index } }
    fn _get_xzrect_uv(&self, p: &Vec3A) -> (f32, f32) { ((p.x - self.x0) / (self.x1 - self.x0), (p.z - self.z0) / (self.z1 - self.z0)) }
}

impl Bounded for XZRectangle {
    fn aabb(&self) -> AABB {
        let min: BVHPoint3 = BVHPoint3::new(self.x0, self.k - utility::EPSILON, self.z0);
        let max: BVHPoint3 = BVHPoint3::new(self.x1, self.k + utility::EPSILON, self.z1);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for XZRectangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for XZRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t: f32 = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max { return None; }
        let xyz: Vec3A = ray.origin() + t * ray.direction();
        if xyz.x < self.x0 || xyz.x > self.x1 || xyz.z < self.z0 || xyz.z > self.z1 { return None; }
        let (u, v) = self._get_xzrect_uv(&xyz);
        let mut rec: HitRecord = HitRecord::new(
            ray.at(t),
            Vec3A::Y,
            self.material.clone(),
            t,
            u,
            v,
            false
        );
        rec.set_face_normal(ray, &rec.normal.clone());
        Some(rec)
    }
    fn is_light(&self) -> bool { self.material.is_light() }
    fn pdf_value(&self, origin: &Point3, v: &Vec3A) -> f32 {
        let t: f32 = (self.k - origin.y) / v.y;
        if t < utility::NEAR_ZERO || t > f32::INFINITY { return 0.0; }
        let xyz: Vec3A = *origin + t * *v;
        if xyz.x < self.x0 || xyz.x > self.x1 || xyz.z < self.z0 || xyz.z > self.z1 { return 0.0; }
        let area: f32 = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared: f32 = t * t * v.length_squared();
        let cosine: f32 = (v.dot(Vec3A::Y) / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, origin: &Point3) -> Vec3A {
        Point3::new(
            utility::random_f32_range(self.x0, self.x1),
            self.k,
            utility::random_f32_range(self.z0, self.z1)
        ) - *origin
    }
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

impl YZRectangle {
    #[allow(dead_code)]
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Box<dyn Material>, node_index: usize) -> Self { YZRectangle { y0, y1, z0, z1, k, material, node_index } }
    fn _get_yzrect_uv(&self, p: &Vec3A) -> (f32, f32) { ((p.y - self.y0) / (self.y1 - self.y0), (p.z - self.z0) / (self.z1 - self.z0)) }
}

impl Bounded for YZRectangle {
    fn aabb(&self) -> AABB {
        let min: BVHPoint3 = BVHPoint3::new(self.k - utility::EPSILON, self.y0, self.z0);
        let max: BVHPoint3 = BVHPoint3::new(self.k + utility::EPSILON, self.y1, self.z1);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for YZRectangle {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for YZRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t: f32 = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max { return None; }
        let xyz: Vec3A = ray.origin() + t * ray.direction();
        if xyz.y < self.y0 || xyz.y > self.y1 || xyz.z < self.z0 || xyz.z > self.z1 { return None; }
        let (u, v) = self._get_yzrect_uv(&xyz);
        let mut rec: HitRecord = HitRecord::new(
            ray.at(t),
            Vec3A::X,
            self.material.clone(),
            t,
            u,
            v,
            false
        );
        rec.set_face_normal(ray, &rec.normal.clone());
        Some(rec)
    }
    fn is_light(&self) -> bool { self.material.is_light() }
    fn pdf_value(&self, origin: &Point3, v: &Vec3A) -> f32 {
        let t: f32 = (self.k - origin.x) / v.x;
        if t < utility::NEAR_ZERO || t > f32::INFINITY { return 0.0; }
        let xyz: Vec3A = *origin + t * *v;
        if xyz.y < self.y0 || xyz.y > self.y1 || xyz.z < self.z0 || xyz.z > self.z1 { return 0.0; } 
        let area: f32 = (self.y1 - self.y0) * (self.z1 - self.z0);
        let distance_squared: f32 = t * t * v.length_squared();
        let cosine: f32 = (v.dot(Vec3A::X) / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, origin: &Point3) -> Vec3A {
        Point3::new(
            self.k,
            utility::random_f32_range(self.y0, self.y1),
            utility::random_f32_range(self.z0, self.z1)
        ) - *origin
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
        assert!(rectangle.hit(&ray, 0.0, 100.0).is_some());
    }
    #[test]
    fn test_xzrectangle_hit() {
        let material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let rectangle: XZRectangle = XZRectangle::new(-1.0, 1.0, -1.0, 1.0, 0.0, material, 0);
        let ray: Ray = Ray::new(Point3::new(0.0, -1.0, 0.0), Vec3A::new(0.0, 1.0, 0.0));
        assert!(rectangle.hit(&ray, 0.0, 100.0).is_some());
    }
    #[test]
    fn test_yzrectangle_hit() {
        let material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let rectangle: YZRectangle = YZRectangle::new(-1.0, 1.0, -1.0, 1.0, 0.0, material, 0);
        let ray: Ray = Ray::new(Point3::new(-1.0, 0.0, 0.0), Vec3A::new(1.0, 0.0, 0.0));
        assert!(rectangle.hit(&ray, 0.0, 100.0).is_some());
    }
    #[test]
    fn test_rectangle_hit() {
        let material: Box<dyn Material> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let rectangle: Rectangle = Rectangle::XYRectangle(XYRectangle::new(-1.0, 1.0, -1.0, 1.0, 0.0, material.clone(), 0));
        let ray: Ray = Ray::new(Point3::new(0.0, 0.0, -1.0), Vec3A::new(0.0, 0.0, 1.0));
        assert!(rectangle.hit(&ray, 0.0, 100.0).is_some());
    }
}
