use std::sync::Arc;

use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::Point3 as BVHPoint3;

use glam;
use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;


trait Translatable: Hittable + Bounded + Sync + Send {}
struct Translate {
    // ptr has to implement the trait Bounded
    ptr: Box<dyn Translatable>,
    //ptr: Box<dyn Hittable + Sync + Send>,
    offset: Vec3A,
    node_index: usize,
}

impl Translate {
    pub fn new(ptr: Box<dyn Translatable>, offset: Vec3A, node_index: usize) -> Translate { Translate { ptr, offset, node_index } }
}

impl Bounded for Translate {
    fn aabb(&self) -> AABB {
        let output_box: AABB = self.ptr.aabb();
        let out_min: BVHPoint3 = BVHPoint3::new(output_box.min.x + self.offset.x, output_box.min.y + self.offset.y, output_box.min.z + self.offset.y);
        let out_max: BVHPoint3 = BVHPoint3::new(output_box.max.x + self.offset.x, output_box.max.y + self.offset.y, output_box.max.z + self.offset.y);
        AABB::with_bounds(out_min, out_max)
    }
}

impl BHShape for Translate {
    fn set_bh_node_index(&mut self, index: usize) { self.node_index = index; }
    fn bh_node_index(&self) -> usize { self.node_index }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut moved_r: Ray = Ray::new(ray.origin() - self.offset, ray.direction());
        let mut temp_rec: HitRecord = HitRecord::empty();
        if !self.ptr.hit(&mut moved_r, t_min, t_max, &mut temp_rec) { return false; }
        temp_rec.p += self.offset;
        rec.set_face_normal(&mut moved_r, &temp_rec.normal);
        rec.p = temp_rec.p;
        rec.t = temp_rec.t;
        rec.mat_ptr = temp_rec.mat_ptr;
        //rec.u = temp_rec.u;
        //rec.v = temp_rec.v;
        rec.front_face = temp_rec.front_face;
        true
    }
}
