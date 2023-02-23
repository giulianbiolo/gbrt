// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Sphere Array struct

use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::{Point3 as BVHPoint3, Vector3 as BVHVector3};
use bvh::ray::Ray as BVHRay;

use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::sphere::Sphere;
use crate::point3::Point3;
use crate::utility;


#[derive(Clone)]
pub struct SphereArray {
    spheres: Vec<Sphere>,
    bvh: BVH,
    node_index: usize,
}

unsafe impl Sync for SphereArray {}
unsafe impl Send for SphereArray {}

impl SphereArray {
    #[allow(dead_code)]
    pub fn new(spheres: &mut Vec<Sphere>) -> SphereArray {
        let mut spheres: Vec<Sphere> = spheres.clone();
        let bvh: BVH = BVH::build(&mut spheres);
        SphereArray { spheres, bvh, node_index: 0 }
    }
}

impl Bounded for SphereArray {
    fn aabb(&self) -> bvh::aabb::AABB {
        let (cmin, cmax) = self.spheres.iter().fold((self.spheres[0].aabb().min, self.spheres[0].aabb().max), |(cmin, cmax), sphere| {
            let sphereaabb = sphere.aabb();
            (cmin.min(sphereaabb.min), cmax.max(sphereaabb.max))
        });
        bvh::aabb::AABB::with_bounds(cmin, cmax)
    }
}

impl BHShape for SphereArray {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for SphereArray {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let bvhray: BVHRay = BVHRay::new(BVHPoint3::new(ray.origin[0], ray.origin[1], ray.origin[2]), BVHVector3::new(ray.direction[0], ray.direction[1], ray.direction[2]));
        let hit_spheres_aabb: Vec<&Sphere> = self.bvh.traverse(&bvhray, &self.spheres);

        hit_spheres_aabb.iter()
        .filter_map(|sphere| sphere.hit(ray, t_min, t_max))
        .filter(|hit| hit.t > t_min && hit.t < t_max)
        .min_by(|hit1, hit2| { hit1.t.partial_cmp(&hit2.t).unwrap() })
    }
    fn is_light(&self) -> bool { false }
    fn pdf_value(&self, origin: &Point3, v: &Vec3A) -> f32 {
        let weight: f32 = 1.0 / (self.spheres.len() as f32);
        self.spheres.iter().map(|sphere| sphere.pdf_value(origin, v) * weight).sum()
    }
    fn random(&self, o: &Point3) -> Vec3A { self.spheres[utility::random_usize_range(0, self.spheres.len())].random(o) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Lambertian;
    use crate::color::Color;
    use crate::point3::Point3;

    #[test]
    fn test_sphere_array_hit() -> Result<(), std::fmt::Error> {
        let mut spheres: Vec<Sphere> = Vec::new();
        spheres.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))), 0));
        spheres.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))), 0));
        let sphere_array: SphereArray = SphereArray::new(&mut spheres);
        let r: Ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, -1.0));
        assert!(sphere_array.hit(&r, 0.0, 100.0).is_some());
        Ok(())
    }
}
