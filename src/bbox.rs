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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
        if tmin > tymax || tymin > tmax { return None; }
        // If the intervals along the x and y axis overlap, the ray hits the box
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }
        // Calculate the interval of possible hit times along the z axis
        let mut tzmin = (self.center.z - self.depth / 2.0 - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.center.z + self.depth / 2.0 - ray.origin.z) / ray.direction.z;
        // Swap tzmin and tzmax if necessary
        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }
        // If the intervals along the x and z axis do not overlap, the ray does not hit the box
        if tmin > tzmax || tzmin > tmax { return None; }
        // If the intervals along the x and z axis overlap, the ray hits the box
        if tzmin > tmin { tmin = tzmin; }
        if tzmax < tmax { tmax = tzmax; }
        // Check if the hit time is within the allowed range
        if tmin < t_max && tmax > t_min {
            let mut t = tmin;
            if t < t_min { t = tmax; }
            if t > t_min && t < t_max {
                let p = ray.at(t);
                Some(HitRecord::new(p, (p - self.center) / Vec3A::new(self.width, self.height, self.depth), self.material.clone(), t, false))
            } else { None }
        } else { None }
    }
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
