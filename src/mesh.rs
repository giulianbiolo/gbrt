// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Mesh struct

use std::collections::HashMap;
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
        // let mut triangles: Vec<Triangle> = Vec::new();
        let objfile = std::fs::File::open(filename).unwrap();
        let input = std::io::BufReader::new(objfile);
        let mut model: Obj = load_obj(input).expect("Failed to load obj");

        let (min, max) = model.vertices.iter().fold(
            (Vec3A::new(INFINITY, INFINITY, INFINITY), Vec3A::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY)),
            |(min, max), v| {
                let ve = Vec3A::new(v.position[0], v.position[1], v.position[2]);
                (min.min(ve), max.max(ve))
            }
        );
        // We want to find the greatest difference between the dimensions
        let diff = (max - min).max_element(); // This is the greatest difference
        // We will scale the object so that the greatest difference is 1
        let scaling_factor: Vec3A = Vec3A::splat(diff).recip() * Vec3A::splat(scaling_factor);
        let rotation_matrix: glam::Mat3A = 
          glam::Mat3A::from_rotation_x(rotation[0].to_radians())
        * glam::Mat3A::from_rotation_y(rotation[1].to_radians())
        * glam::Mat3A::from_rotation_z(rotation[2].to_radians());
        for vertex in model.vertices.iter_mut() {
            let mut v: Vec3A = Vec3A::new(vertex.position[0], vertex.position[1], vertex.position[2]);
            v = v * scaling_factor;
            v = v + position;
            v = rotation_matrix.mul(v);
            vertex.position = [v.x, v.y, v.z];
        }

        // ! This hasn't yet been tested, it's been copied from the stl loader, that one works
        // TODO: Test obj loader and see if normals are being computed correctly
        let mut normals_hash: HashMap<usize, Vec3A> = HashMap::new();
        for face in model.indices.chunks(3) {
            let v0: Vec3A = Vec3A::new(model.vertices[face[0] as usize].position[0], model.vertices[face[0] as usize].position[1], model.vertices[face[0] as usize].position[2]);
            let v1: Vec3A = Vec3A::new(model.vertices[face[1] as usize].position[0], model.vertices[face[1] as usize].position[1], model.vertices[face[1] as usize].position[2]);
            let v2: Vec3A = Vec3A::new(model.vertices[face[2] as usize].position[0], model.vertices[face[2] as usize].position[1], model.vertices[face[2] as usize].position[2]);
            let normal: Vec3A = (v1 - v0).cross(v2 - v0).normalize();
            normals_hash.insert(face[0] as usize, normal);
            normals_hash.insert(face[1] as usize, normal);
            normals_hash.insert(face[2] as usize, normal);
        }
        // We then return the triangles
        (0..model.indices.len()).step_by(3).fold(
            Vec::new(),
            |mut triangles, idx| {
                let v0: Vec3A = Vec3A::new(model.vertices[model.indices[idx] as usize].position[0], model.vertices[model.indices[idx] as usize].position[1], model.vertices[model.indices[idx] as usize].position[2]);
                let v1: Vec3A = Vec3A::new(model.vertices[model.indices[idx + 1] as usize].position[0], model.vertices[model.indices[idx + 1] as usize].position[1], model.vertices[model.indices[idx + 1] as usize].position[2]);
                let v2: Vec3A = Vec3A::new(model.vertices[model.indices[idx + 2] as usize].position[0], model.vertices[model.indices[idx + 2] as usize].position[1], model.vertices[model.indices[idx + 2] as usize].position[2]);
                // Check whether the triangle is degenerate
                // if (v0 - v1).length_squared() < EPSILON || (v1 - v2).length_squared() < EPSILON || (v2 - v0).length_squared() < EPSILON { return triangles; }
                let n0: Vec3A = normals_hash[&(model.indices[idx] as usize)];
                let n1: Vec3A = normals_hash[&(model.indices[idx + 1] as usize)];
                let n2: Vec3A = normals_hash[&(model.indices[idx + 2] as usize)];
                let normals: Box<[Vec3A; 3]> = Box::new([n0, n1, n2]);
                triangles.push(Triangle::new(Box::new([v0, v1, v2]), normals, material.clone(), 0));
                triangles
            }
        )
    }
    fn _load_stl_triangles(position: Point3, scaling_factor: f32, rotation: Vec3A, filename: &str, material: Box<dyn Material>) -> Vec<Triangle> {
        let mut stlfile = std::fs::OpenOptions::new().read(true).open(filename).unwrap();
        let mut stl = stl_io::read_stl(&mut stlfile).unwrap();
        // let mut triangles: Vec<Triangle> = Vec::new();
        let (min, max) = stl.vertices.iter().fold(
            (Vec3A::new(INFINITY, INFINITY, INFINITY), Vec3A::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY)),
            |(min, max), v| {
                let v = Vec3A::new(v[0], v[1], v[2]);
                (min.min(v), max.max(v))
            }
        );
        let diff = (max - min).max_element(); // This is the greatest difference
        let scaling_factor: Vec3A = Vec3A::splat(diff).recip() * Vec3A::splat(scaling_factor);
        let rotation_matrix: glam::Mat3A = 
          glam::Mat3A::from_rotation_x(rotation[0].to_radians())
        * glam::Mat3A::from_rotation_y(rotation[1].to_radians())
        * glam::Mat3A::from_rotation_z(rotation[2].to_radians());
        for vertex in stl.vertices.iter_mut() {
            let mut v: Vec3A = Vec3A::new(vertex[0], vertex[1], vertex[2]);
            v = v * scaling_factor;
            v = v + position;
            v = rotation_matrix.mul(v);
            *vertex = Vector::new([v.x, v.y, v.z]);
        }
        // Normals is an array of length equal to the number of vertices
        let mut normals_hash: HashMap<usize, Vec3A> = HashMap::new();
        for face in stl.faces.iter() {
            let v0: Vec3A = Vec3A::new(stl.vertices[face.vertices[0] as usize][0], stl.vertices[face.vertices[0] as usize][1], stl.vertices[face.vertices[0] as usize][2]);
            let v1: Vec3A = Vec3A::new(stl.vertices[face.vertices[1] as usize][0], stl.vertices[face.vertices[1] as usize][1], stl.vertices[face.vertices[1] as usize][2]);
            let v2: Vec3A = Vec3A::new(stl.vertices[face.vertices[2] as usize][0], stl.vertices[face.vertices[2] as usize][1], stl.vertices[face.vertices[2] as usize][2]);
            let normal: Vec3A = ((v1 - v0).cross(v2 - v0)).normalize();
            for vertex in face.vertices.iter() {
                let normal0: &mut Vec3A = normals_hash.entry(*vertex as usize).or_insert(Vec3A::ZERO);
                *normal0 += normal;
            }
        }
        for normal in normals_hash.values_mut() { *normal = normal.normalize(); }

        // We then return the triangles
        stl.faces.iter().map(|face|{
            let v0: Vec3A = Vec3A::new(stl.vertices[face.vertices[0] as usize][0], stl.vertices[face.vertices[0] as usize][1], stl.vertices[face.vertices[0] as usize][2]);
            let v1: Vec3A = Vec3A::new(stl.vertices[face.vertices[1] as usize][0], stl.vertices[face.vertices[1] as usize][1], stl.vertices[face.vertices[1] as usize][2]);
            let v2: Vec3A = Vec3A::new(stl.vertices[face.vertices[2] as usize][0], stl.vertices[face.vertices[2] as usize][1], stl.vertices[face.vertices[2] as usize][2]);
            let normals: Box<[Vec3A; 3]> = Box::new([
                normals_hash[&(face.vertices[0] as usize)],
                normals_hash[&(face.vertices[1] as usize)],
                normals_hash[&(face.vertices[2] as usize)],
            ]);
            Triangle::new(Box::new([v0, v1, v2]), normals, material.clone(), 0)
        })
        .filter(|triangle| triangle.check_not_degenerate())
        .collect()
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let bvhray: BVHRay = BVHRay::new(BVHPoint3::new(ray.origin[0], ray.origin[1], ray.origin[2]), BVHVector3::new(ray.direction[0], ray.direction[1], ray.direction[2]));
        let hit_triangles_aabbs: Vec<&Triangle> = self.bvh.traverse(&bvhray, &self.triangles);
        hit_triangles_aabbs.iter()
        .filter_map(|triangle| triangle.hit(ray, t_min, t_max))
        .filter(|hit| hit.t > t_min && hit.t < t_max)
        .min_by(|hit1, hit2| { hit1.t.partial_cmp(&hit2.t).unwrap() })
    }
}
