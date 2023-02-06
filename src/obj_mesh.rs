// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Object Mesh struct

use std::ops::Mul;

use glam;
use glam::Vec3A;

use stl_io;

use bvh::bvh::BVH;
use bvh::{Point3 as BVHPoint3, Vector3 as BVHVector3};
use bvh::ray::Ray as BVHRay;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;
use crate::triangle::Triangle;
use crate::bbox::BBox;
use crate::utility::{INFINITY, NEG_INFINITY};


#[derive(Clone)]
pub struct ObjMesh {
    triangles: Vec<Triangle>,
    bvh: BVH,
}

unsafe impl Sync for ObjMesh {}
unsafe impl Send for ObjMesh {}

impl ObjMesh {
    #[allow(dead_code)]
    pub fn new(position: Point3, dimensions: Vec3A, rotation: Vec3A, filename: &str, material: Box<dyn Material>) -> ObjMesh {
        let mut stlfile = std::fs::OpenOptions::new().read(true).open(filename).unwrap();
        let stl = stl_io::read_stl(&mut stlfile).unwrap();
        let mut triangles: Vec<Triangle> = Vec::new();

        let mut min_x = INFINITY;
        let mut min_y = INFINITY;
        let mut min_z = INFINITY;
        let mut max_x = NEG_INFINITY;
        let mut max_y = NEG_INFINITY;
        let mut max_z = NEG_INFINITY;
        let mut median_x = 0.0;
        let mut median_y = 0.0;
        let mut median_z = 0.0;
        for i in 0..stl.vertices.len() {
            let v = stl.vertices[i];
            if v[0] < min_x { min_x = v[0]; }
            if v[1] < min_y { min_y = v[1]; }
            if v[2] < min_z { min_z = v[2]; }
            if v[0] > max_x { max_x = v[0]; }
            if v[1] > max_y { max_y = v[1]; }
            if v[2] > max_z { max_z = v[2]; }
            median_x += v[0];
            median_y += v[1];
            median_z += v[2];
        }
        let inv_obj_width: f32 = 1.0 / (max_x - min_x);
        let inv_obj_height: f32 = 1.0 / (max_y - min_y);
        let inv_obj_depth: f32 = 1.0 / (max_z - min_z);
        median_x /= stl.vertices.len() as f32;
        median_y /= stl.vertices.len() as f32;
        median_z /= stl.vertices.len() as f32;
        let median_vector: Vec3A = Vec3A::new(median_x, median_y, median_z);
        // We want to find a factor that will reposition the object to the center of the bounding box and scale it to fit the bounding box
        let scaling_factor: Vec3A = (dimensions * Vec3A::new(inv_obj_width, inv_obj_height, inv_obj_depth)) / 2.0;
        let rotation_matrix: glam::Mat3A = glam::Mat3A::from_rotation_x(rotation[0].to_radians()) * glam::Mat3A::from_rotation_y(rotation[1].to_radians()) * glam::Mat3A::from_rotation_z(rotation[2].to_radians());
        // println!("Size of the object: {} x {} x {}", max_x - min_x, max_y - min_y, max_z - min_z);
        // println!("Inv size of the object: {} x {} x {}", inv_obj_width, inv_obj_height, inv_obj_depth);

        for face in stl.faces {
            let v0 = stl.vertices[face.vertices[0] as usize];
            let v1 = stl.vertices[face.vertices[1] as usize];
            let v2 = stl.vertices[face.vertices[2] as usize];
            let v0: Vec3A = rotation_matrix.mul((Vec3A::new(v0[0], v0[1], v0[2]) - median_vector).mul(scaling_factor) + position);
            let v1: Vec3A = rotation_matrix.mul((Vec3A::new(v1[0], v1[1], v1[2]) - median_vector).mul(scaling_factor) + position);
            let v2: Vec3A = rotation_matrix.mul((Vec3A::new(v2[0], v2[1], v2[2]) - median_vector).mul(scaling_factor) + position);
            triangles.push(Triangle::new([v0, v1, v2], material.clone(), 0));
        }
        let bvh: BVH = BVH::build(&mut triangles);
        ObjMesh { triangles, bvh }
    }
}

impl Hittable for ObjMesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let bvhray: BVHRay = BVHRay::new(BVHPoint3::new(ray.origin[0], ray.origin[1], ray.origin[2]), BVHVector3::new(ray.direction[0], ray.direction[1], ray.direction[2]));
        let hit_triangles_aabbs: Vec<&Triangle> = self.bvh.traverse(&bvhray, &self.triangles);
        
        let mut temp_rec = HitRecord::empty();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for triangle in hit_triangles_aabbs {
            if triangle.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
