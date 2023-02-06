// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Sphere Array struct

use bvh::bvh::BVH;
use bvh::{Point3 as BVHPoint3, Vector3 as BVHVector3};
use bvh::ray::Ray as BVHRay;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::sphere::Sphere;



#[derive(Clone)]
pub struct SphereArray {
    spheres: Vec<Sphere>,
    bvh: BVH,
}

unsafe impl Sync for SphereArray {}
unsafe impl Send for SphereArray {}

impl SphereArray {
    #[allow(dead_code)]
    pub fn new(spheres: &mut Vec<Sphere>) -> SphereArray {
        let mut spheres: Vec<Sphere> = spheres.clone();
        let bvh: BVH = BVH::build(&mut spheres);
        SphereArray { spheres, bvh }
    }
}

impl Hittable for SphereArray {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let bvhray: BVHRay = BVHRay::new(BVHPoint3::new(ray.origin[0], ray.origin[1], ray.origin[2]), BVHVector3::new(ray.direction[0], ray.direction[1], ray.direction[2]));
        let hit_spheres_aabb: Vec<&Sphere> = self.bvh.traverse(&bvhray, &self.spheres);

        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for sphere in hit_spheres_aabb {
            // we want to find the closest hit
            if sphere.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
