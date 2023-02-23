// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the BBox struct

use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::{Point3 as BVHPoint3, Vector3 as BVHVector3};
use bvh::ray::Ray as BVHRay;

use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;
use crate::rectangle::{XZRectangle, YZRectangle, Rectangle, XYRectangle};
use crate::utility;


pub struct BBox {
    faces: Vec<Rectangle>,
    bvh: BVH,
    node_index: usize,
}

unsafe impl Sync for BBox {}
unsafe impl Send for BBox {}

impl BBox {
    #[allow(dead_code)]
    pub fn new(center: Point3, dimensions: Vec3A, material: Box<dyn Material>) -> BBox {
        let mut faces: Vec<Rectangle> = Vec::with_capacity(6);
        faces.push(Rectangle::XYRectangle(XYRectangle::new(center.x - dimensions.x / 2.0, center.x + dimensions.x / 2.0, center.y - dimensions.y / 2.0, center.y + dimensions.y / 2.0, center.z - dimensions.z / 2.0, material.clone(), 0)));
        faces.push(Rectangle::XYRectangle(XYRectangle::new(center.x - dimensions.x / 2.0, center.x + dimensions.x / 2.0, center.y - dimensions.y / 2.0, center.y + dimensions.y / 2.0, center.z + dimensions.z / 2.0, material.clone(), 0)));
        faces.push(Rectangle::XZRectangle(XZRectangle::new(center.x - dimensions.x / 2.0, center.x + dimensions.x / 2.0, center.z - dimensions.z / 2.0, center.z + dimensions.z / 2.0, center.y - dimensions.y / 2.0, material.clone(), 0)));
        faces.push(Rectangle::XZRectangle(XZRectangle::new(center.x - dimensions.x / 2.0, center.x + dimensions.x / 2.0, center.z - dimensions.z / 2.0, center.z + dimensions.z / 2.0, center.y + dimensions.y / 2.0, material.clone(), 0)));
        faces.push(Rectangle::YZRectangle(YZRectangle::new(center.y - dimensions.y / 2.0, center.y + dimensions.y / 2.0, center.z - dimensions.z / 2.0, center.z + dimensions.z / 2.0, center.x - dimensions.x / 2.0, material.clone(), 0)));
        faces.push(Rectangle::YZRectangle(YZRectangle::new(center.y - dimensions.y / 2.0, center.y + dimensions.y / 2.0, center.z - dimensions.z / 2.0, center.z + dimensions.z / 2.0, center.x + dimensions.x / 2.0, material.clone(), 0)));
        let bvh: BVH = BVH::build(&mut faces);
        BBox {
            faces,
            bvh,
            node_index: 0,
        }
    }
}

impl Bounded for BBox {
    fn aabb(&self) -> bvh::aabb::AABB {
        let (min, max) = self.faces.iter().fold((BVHPoint3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY), BVHPoint3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY)), |(min, max), face| {
            let face_aabb: bvh::aabb::AABB = face.aabb();
            (min.min(face_aabb.min), max.max(face_aabb.max))    
        });
        bvh::aabb::AABB::with_bounds(min, max)
    }
}

impl BHShape for BBox {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for BBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let bvhray: BVHRay = BVHRay::new(BVHPoint3::new(ray.origin[0], ray.origin[1], ray.origin[2]), BVHVector3::new(ray.direction[0], ray.direction[1], ray.direction[2]));
        let hit_faces_aabb: Vec<&Rectangle> = self.bvh.traverse(&bvhray, &self.faces);

        hit_faces_aabb.iter()
        .filter_map(|face| face.hit(ray, t_min, t_max))
        .filter(|hit| hit.t > t_min && hit.t < t_max)
        .min_by(|hit1, hit2| { hit1.t.partial_cmp(&hit2.t).unwrap() })
    }
    fn is_light(&self) -> bool { self.faces.iter().any(|face| face.is_light()) }
    fn pdf_value(&self, origin: &Point3, v: &Vec3A) -> f32 {
        let weight: f32 = 1.0 / self.faces.len() as f32;
        self.faces.iter().map(|triangle| triangle.pdf_value(origin, v) * weight).sum()
    }
    fn random(&self, o: &Point3) -> Vec3A { self.faces[utility::random_usize_range(0, self.faces.len())].random(o) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::material::Lambertian;

    #[test]
    fn test_bbox() -> Result<(), String> {
        let center: Point3 = Point3::new(0.0, 0.0, 0.0);
        let dimensions: Vec3A = Vec3A::new(1.0, 1.0, 1.0);
        let material: Box<Lambertian> = Box::new(Lambertian::new(Color::new(0.0, 0.0, 0.0)));
        let bbox: BBox = BBox::new(center, dimensions, material);
        let ray: Ray = Ray::new(Point3::new(0.0, 0.0, -2.0), Vec3A::new(0.0, 0.0, 1.0));
        assert!(bbox.hit(&ray, 0.0, 100.0).is_some());
        Ok(())
    }
}
