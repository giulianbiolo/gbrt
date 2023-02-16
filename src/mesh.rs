// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Mesh struct

use std::ops::Mul;

use stl_io::{self, Vector};
use obj::{load_obj, Obj};

use bvh::bvh::BVH;
use bvh::{Point3 as BVHPoint3, Vector3 as BVHVector3};
use bvh::ray::Ray as BVHRay;

use glam::Vec3A;

use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::hittable_list::Hittable;
use crate::material::Material;
use crate::point3::Point3;
use crate::triangle::Triangle;
use crate::utility::{INFINITY, NEG_INFINITY};


#[derive(Clone)]
pub struct Mesh {
    triangles: Vec<Triangle>,
    bvh: BVH,
}

unsafe impl Sync for Mesh {}
unsafe impl Send for Mesh {}

impl Mesh {
    #[allow(dead_code)]
    pub fn new(position: Point3, scaling_factor: f32, rotation: Vec3A, filename: &str, material: Box<dyn Material>) -> Mesh {
        let mut triangles: Vec<Triangle>;
        match filename.split('.').last().unwrap() {
            "stl" => triangles = Mesh::_load_stl_triangles(position, scaling_factor, rotation, filename, material),
            "obj" => triangles = Mesh::_load_obj_triangles(position, scaling_factor, rotation, filename, material),
            _ => panic!("File format not supported for: {}", filename),
        };
        let bvh: BVH = BVH::build(&mut triangles);
        Mesh { triangles, bvh }
    }
    fn _load_obj_triangles(position: Point3, scaling_factor: f32, rotation: Vec3A, filename: &str, material: Box<dyn Material>) -> Vec<Triangle> {
        let mut triangles: Vec<Triangle> = Vec::new();
        let objfile = std::fs::File::open(filename).unwrap();
        let input = std::io::BufReader::new(objfile);
        let mut model: Obj = load_obj(input).expect("Failed to load obj");
        // model.vertices, model.indices
        // println!("Model: {:?}", model);
        let (/*center,*/ min, max) = model.vertices.iter().fold(
            (/*Vec3A::ZERO,*/ Vec3A::new(INFINITY, INFINITY, INFINITY), Vec3A::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY)),
            |(/*acc,*/ min, max), v| {
                let ve = Vec3A::new(v.position[0], v.position[1], v.position[2]);
                (/*acc + ve,*/ min.min(ve), max.max(ve))
            }
        );
        // let center: Vec3A = center / model.vertices.len() as f32;
        // We want to find the greatest difference between the dimensions
        let diff = (max - min).max_element(); // This is the greatest difference
        // We will scale the object so that the greatest difference is 1
        let scaling_factor: Vec3A = Vec3A::splat(diff).recip() * Vec3A::splat(scaling_factor);
        for vertex in model.vertices.iter_mut() {
            let mut v: Vec3A = Vec3A::new(vertex.position[0], vertex.position[1], vertex.position[2]);
            // v = v - center;
            v = v * scaling_factor;
            v = v + position;
            vertex.position = [v.x, v.y, v.z];
        }
        let rotation_matrix: glam::Mat3A = 
          glam::Mat3A::from_rotation_x(rotation[0].to_radians())
        * glam::Mat3A::from_rotation_y(rotation[1].to_radians())
        * glam::Mat3A::from_rotation_z(rotation[2].to_radians());
        // Here we need to find the triangles knowing the indices
        for idx in (0..model.indices.len()).step_by(3) {
            let mut v0: Vec3A = Vec3A::new(model.vertices[model.indices[idx] as usize].position[0], model.vertices[model.indices[idx] as usize].position[1], model.vertices[model.indices[idx] as usize].position[2]);
            let mut v1: Vec3A = Vec3A::new(model.vertices[model.indices[idx + 1] as usize].position[0], model.vertices[model.indices[idx + 1] as usize].position[1], model.vertices[model.indices[idx + 1] as usize].position[2]);
            let mut v2: Vec3A = Vec3A::new(model.vertices[model.indices[idx + 2] as usize].position[0], model.vertices[model.indices[idx + 2] as usize].position[1], model.vertices[model.indices[idx + 2] as usize].position[2]);
            v0 = rotation_matrix.mul(v0 );
            v1 = rotation_matrix.mul(v1 );
            v2 = rotation_matrix.mul(v2 );
            triangles.push(Triangle::new([v0, v1, v2], material.clone(), 0));
        }
        triangles
    }
    fn _load_stl_triangles(position: Point3, scaling_factor: f32, rotation: Vec3A, filename: &str, material: Box<dyn Material>) -> Vec<Triangle> {
        let mut stlfile = std::fs::OpenOptions::new().read(true).open(filename).unwrap();
        let mut stl = stl_io::read_stl(&mut stlfile).unwrap();
        let mut triangles: Vec<Triangle> = Vec::new();
        let (/*center,*/ min, max) = stl.vertices.iter().fold(
            (/*Vec3A::ZERO,*/ Vec3A::new(INFINITY, INFINITY, INFINITY), Vec3A::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY)),
            |(/*acc,*/ min, max), v| {
                let v = Vec3A::new(v[0], v[1], v[2]);
                (/*acc + v,*/ min.min(v), max.max(v))
            }
        );
        // let center: Vec3A = center / stl.vertices.len() as f32;
        let diff = (max - min).max_element(); // This is the greatest difference
        let scaling_factor: Vec3A = Vec3A::splat(diff).recip() * Vec3A::splat(scaling_factor);
        for vertex in stl.vertices.iter_mut() {
            let mut v: Vec3A = Vec3A::new(vertex[0], vertex[1], vertex[2]);
            // v = v - center;
            v = v * scaling_factor;
            v = v + position;
            *vertex = Vector::new([v.x, v.y, v.z]);
        }
        let rotation_matrix: glam::Mat3A = 
          glam::Mat3A::from_rotation_x(rotation[0].to_radians())
        * glam::Mat3A::from_rotation_y(rotation[1].to_radians())
        * glam::Mat3A::from_rotation_z(rotation[2].to_radians());
        for face in stl.faces {
            let v0: Vector<f32> = stl.vertices[face.vertices[0] as usize];
            let v1: Vector<f32> = stl.vertices[face.vertices[1] as usize];
            let v2: Vector<f32> = stl.vertices[face.vertices[2] as usize];
            let v0: Vec3A = rotation_matrix.mul(Vec3A::new(v0[0], v0[1], v0[2]) );
            let v1: Vec3A = rotation_matrix.mul(Vec3A::new(v1[0], v1[1], v1[2]) );
            let v2: Vec3A = rotation_matrix.mul(Vec3A::new(v2[0], v2[1], v2[2]) );
            triangles.push(Triangle::new([v0, v1, v2], material.clone(), 0));
        }
        triangles
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let bvhray: BVHRay = BVHRay::new(BVHPoint3::new(ray.origin[0], ray.origin[1], ray.origin[2]), BVHVector3::new(ray.direction[0], ray.direction[1], ray.direction[2]));
        let hit_triangles_aabbs: Vec<&Triangle> = self.bvh.traverse(&bvhray, &self.triangles);

        hit_triangles_aabbs.iter()
        .filter_map(|triangle| triangle.hit(ray, t_min, t_max))
        .min_by(|hit1, hit2| { hit1.t.partial_cmp(&hit2.t).unwrap() })
    }
}
