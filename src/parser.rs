// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 09/02/2023
// Description: This file implements the parsing of YAML config files

use yaml_rust::{YamlLoader, Yaml};

use glam;
use glam::Vec3A;

use crate::material::DiffuseLight;
use crate::hittable_list::HittableList;
use crate::hittable_list::Hittable;
use crate::sphere::Sphere;
use crate::triangle::Triangle;
use crate::rectangle::{XYRectangle, XZRectangle, YZRectangle};
use crate::mesh::Mesh;
use crate::material::{Material, Lambertian, Metal, Dielectric};
use crate::camera::Camera;
use crate::sphere_array::SphereArray;
use crate::utility;
use crate::color::Color;
use crate::point3::Point3;


pub fn parse_yaml_constants(filename: &str) -> utility::Constants {
    if filename == "" { utility::Constants::default() }
    else {
        let content: String = std::fs::read_to_string(filename).unwrap();
        let docs: Vec<Yaml> = YamlLoader::load_from_str(&content).unwrap();
        let hashconsts = docs[0].as_hash().unwrap()[&yaml_rust::Yaml::String("constants".to_string())].as_hash().unwrap().clone();
        let width = hashconsts[&yaml_rust::Yaml::String("width".to_string())].as_i64().unwrap() as u32;
        let height = hashconsts[&yaml_rust::Yaml::String("height".to_string())].as_i64().unwrap() as u32;
        let samples_per_pixel = hashconsts[&yaml_rust::Yaml::String("samplesPerPixel".to_string())].as_i64().unwrap() as u32;
        let max_depth = hashconsts[&yaml_rust::Yaml::String("maxDepth".to_string())].as_i64().unwrap() as u32;
        let aspect_ratio = width as f32 / height as f32;
        utility::Constants {width, height, aspect_ratio, samples_per_pixel, max_depth}
    }
}

pub fn parse_yaml_camera(filename: &str) -> Camera {
    let content: String = std::fs::read_to_string(filename).unwrap();
    let docs: Vec<Yaml> = YamlLoader::load_from_str(&content).unwrap();
    let hashcam = docs[0].as_hash().unwrap()[&yaml_rust::Yaml::String("camera".to_string())].as_hash().unwrap().clone();
    let lookfrom = hashcam[&yaml_rust::Yaml::String("lookFrom".to_string())].as_vec().unwrap();
    let lookat = hashcam[&yaml_rust::Yaml::String("lookAt".to_string())].as_vec().unwrap();
    let vup = hashcam[&yaml_rust::Yaml::String("vup".to_string())].as_vec().unwrap();
    let vfov = hashcam[&yaml_rust::Yaml::String("vfov".to_string())].as_f64().unwrap();
    let aspect_ratio = hashcam[&yaml_rust::Yaml::String("aspectRatio".to_string())].as_f64().unwrap();
    let aperture = hashcam[&yaml_rust::Yaml::String("aperture".to_string())].as_f64().unwrap();
    let focus_dist = hashcam[&yaml_rust::Yaml::String("focusDistance".to_string())].as_f64().unwrap();
    Camera::new(
        &Vec3A::new(lookfrom[0].as_f64().unwrap() as f32, lookfrom[1].as_f64().unwrap() as f32, lookfrom[2].as_f64().unwrap() as f32),
        &Vec3A::new(lookat[0].as_f64().unwrap() as f32, lookat[1].as_f64().unwrap() as f32, lookat[2].as_f64().unwrap() as f32),
        &Vec3A::new(vup[0].as_f64().unwrap() as f32, vup[1].as_f64().unwrap() as f32, vup[2].as_f64().unwrap() as f32),
        vfov as f32,
        aspect_ratio as f32,
        aperture as f32,
        focus_dist as f32
    )
}

pub fn parse_yaml_scene(filename: &str) -> HittableList {
    let mut world: HittableList = HittableList::new();
    let content: String = std::fs::read_to_string(filename).unwrap();
    let docs: Vec<Yaml> = YamlLoader::load_from_str(&content).unwrap();
    let hashworld = docs[0].as_hash().unwrap()[&yaml_rust::Yaml::String("world".to_string())].as_vec().unwrap();
    for hashobj in hashworld {
        let hashobj = hashobj.as_hash().unwrap();
        let objtype = hashobj[&yaml_rust::Yaml::String("objType".to_string())].as_str().unwrap();
        if !objtype.contains("Array") {
            let material: Box<dyn Material + Send + Sync> = _parse_material(hashobj);
            let obj: Box<dyn Hittable + Send + Sync> = _parse_geometry(hashobj, material);
            world.push(obj);
        } else if objtype.contains("Sphere") {
            // * SphereArray *
            // ! In future we will support other objects
            let objects = hashobj[&yaml_rust::Yaml::String("objects".to_string())].as_vec().unwrap();
            let mut spheres: Vec<Sphere> = Vec::<Sphere>::new();
            for obj in objects {
                let obj = obj.as_hash().unwrap();
                let objtype = obj[&yaml_rust::Yaml::String("objType".to_string())].as_str().unwrap();
                if objtype == "Sphere" {
                    let center = obj[&yaml_rust::Yaml::String("center".to_string())].as_vec().unwrap();
                    let radius = obj[&yaml_rust::Yaml::String("radius".to_string())].as_f64().unwrap();
                    let material = _parse_material(obj);
                    spheres.push(Sphere::new(Point3::new(center[0].as_f64().unwrap() as f32, center[1].as_f64().unwrap() as f32, center[2].as_f64().unwrap() as f32), radius as f32, material, 0));
                }
            }
            let spherearray = SphereArray::new(&mut spheres);
            world.push(Box::new(spherearray));
        } else { panic!("Unsupported object type: {}", objtype) }
    }
    world
}

fn _parse_material(hashobj: &yaml_rust::yaml::Hash) -> Box<dyn Material + Send + Sync> {
    let objmat = hashobj[&yaml_rust::Yaml::String("material".to_string())].as_hash().unwrap();
    let objmattype = objmat[&yaml_rust::Yaml::String("matType".to_string())].as_str().unwrap();
    match objmattype {
        "Lambertian" => {
            // has just an albedo
            let albedo = objmat[&yaml_rust::Yaml::String("albedo".to_string())].as_vec().unwrap();
            Box::new(Lambertian::new(Color::new(albedo[0].as_f64().unwrap() as f32, albedo[1].as_f64().unwrap() as f32, albedo[2].as_f64().unwrap() as f32)))
        },
        "Metal" => {
            // has an albedo and a fuzz
            let albedo = objmat[&yaml_rust::Yaml::String("albedo".to_string())].as_vec().unwrap();
            let fuzz = objmat[&yaml_rust::Yaml::String("fuzz".to_string())].as_f64().unwrap();
            Box::new(Metal::new(Color::new(albedo[0].as_f64().unwrap() as f32, albedo[1].as_f64().unwrap() as f32, albedo[2].as_f64().unwrap() as f32), fuzz as f32))
        },
        "Dielectric" => {
            // has just an index of refraction
            let ior = objmat[&yaml_rust::Yaml::String("refractionIdx".to_string())].as_f64().unwrap();
            Box::new(Dielectric::new(ior as f32))
        },
        "DiffuseLight" => {
            // has just an emittance
            let emittance = objmat[&yaml_rust::Yaml::String("emittance".to_string())].as_vec().unwrap();
            Box::new(DiffuseLight::new(Color::new(emittance[0].as_f64().unwrap() as f32, emittance[1].as_f64().unwrap() as f32, emittance[2].as_f64().unwrap() as f32)))
        }
        _ => { panic!("Unknown material type: {:?}", objmat); }
    }
}

fn _parse_geometry(hashobj: &yaml_rust::yaml::Hash, material: Box<dyn Material + Send + Sync>) -> Box<dyn Hittable + Send + Sync> {
    let objtype = hashobj[&yaml_rust::Yaml::String("objType".to_string())].as_str().unwrap();
    match objtype {
        "Sphere" => {
            // has a center and radius
            let center = hashobj[&yaml_rust::Yaml::String("center".to_string())].as_vec().unwrap();
            let radius = hashobj[&yaml_rust::Yaml::String("radius".to_string())].as_f64().unwrap();
            Box::new(Sphere::new(Point3::new(center[0].as_f64().unwrap() as f32, center[1].as_f64().unwrap() as f32, center[2].as_f64().unwrap() as f32), radius as f32, material, 0))
        },
        "Triangle" => {
            // has an array of 3 arrays (the vertices)
            let vertices = hashobj[&yaml_rust::Yaml::String("vertices".to_string())].as_vec().unwrap();
            let v0 = vertices[0].as_vec().unwrap();
            let v1 = vertices[1].as_vec().unwrap();
            let v2 = vertices[2].as_vec().unwrap();
            // obj = Box::new(Triangle::new(Point3::new(v0[0].as_f64().unwrap() as f32, v0[1].as_f64().unwrap() as f32, v0[2].as_f64().unwrap() as f32), Point3::new(v1[0].as_f64().unwrap() as f32, v1[1].as_f64().unwrap() as f32, v1[2].as_f64().unwrap() as f32), Point3::new(v2[0].as_f64().unwrap() as f32, v2[1].as_f64().unwrap() as f32, v2[2].as_f64().unwrap() as f32), material, 0));
            Box::new(
                Triangle::new(
                    [
                        Vec3A::new(v0[0].as_f64().unwrap() as f32, v0[1].as_f64().unwrap() as f32, v0[2].as_f64().unwrap() as f32),
                        Vec3A::new(v1[0].as_f64().unwrap() as f32, v1[1].as_f64().unwrap() as f32, v1[2].as_f64().unwrap() as f32),
                        Vec3A::new(v2[0].as_f64().unwrap() as f32, v2[1].as_f64().unwrap() as f32, v2[2].as_f64().unwrap() as f32)
                    ],
                    material,
                    0
                )
            )
        },
        "XYRectangle" => {
            // has a position, width and height
            let position = hashobj[&yaml_rust::Yaml::String("position".to_string())].as_vec().unwrap();
            let width = hashobj[&yaml_rust::Yaml::String("width".to_string())].as_f64().unwrap();
            let height = hashobj[&yaml_rust::Yaml::String("height".to_string())].as_f64().unwrap();
            Box::new(XYRectangle::new(
                position[0].as_f64().unwrap() as f32 - width as f32 / 2.0,
                position[0].as_f64().unwrap() as f32 + width as f32 / 2.0,
                position[1].as_f64().unwrap() as f32 - height as f32 / 2.0,
                position[1].as_f64().unwrap() as f32 + height as f32 / 2.0,
                position[2].as_f64().unwrap() as f32,
                material,
                0
            ))
        },
        "XZRectangle" => {
            // has a position, width and height
            let position = hashobj[&yaml_rust::Yaml::String("position".to_string())].as_vec().unwrap();
            let width = hashobj[&yaml_rust::Yaml::String("width".to_string())].as_f64().unwrap();
            let height = hashobj[&yaml_rust::Yaml::String("height".to_string())].as_f64().unwrap();
            Box::new(XZRectangle::new(
                position[0].as_f64().unwrap() as f32 - width as f32 / 2.0,
                position[0].as_f64().unwrap() as f32 + width as f32 / 2.0,
                position[2].as_f64().unwrap() as f32 - height as f32 / 2.0,
                position[2].as_f64().unwrap() as f32 + height as f32 / 2.0,
                position[1].as_f64().unwrap() as f32,
                material,
                0
            ))
        },
        "YZRectangle" => {
            // has a position, width and height
            let position = hashobj[&yaml_rust::Yaml::String("position".to_string())].as_vec().unwrap();
            let width = hashobj[&yaml_rust::Yaml::String("width".to_string())].as_f64().unwrap();
            let height = hashobj[&yaml_rust::Yaml::String("height".to_string())].as_f64().unwrap();
            Box::new(YZRectangle::new(
                position[1].as_f64().unwrap() as f32 - width as f32 / 2.0,
                position[1].as_f64().unwrap() as f32 + width as f32 / 2.0,
                position[2].as_f64().unwrap() as f32 - height as f32 / 2.0,
                position[2].as_f64().unwrap() as f32 + height as f32 / 2.0,
                position[0].as_f64().unwrap() as f32,
                material,
                0
            ))
        },
        "Mesh" => {
            // has a filename, position, rotation and scale
            let filename = hashobj[&yaml_rust::Yaml::String("filename".to_string())].as_str().unwrap();
            let position = hashobj[&yaml_rust::Yaml::String("position".to_string())].as_vec().unwrap();
            let rotation = hashobj[&yaml_rust::Yaml::String("rotation".to_string())].as_vec().unwrap();
            let scale = hashobj[&yaml_rust::Yaml::String("scalingFactor".to_string())].as_f64().unwrap();
            Box::new(
                Mesh::new(
                    Vec3A::new(position[0].as_f64().unwrap() as f32, position[1].as_f64().unwrap() as f32, position[2].as_f64().unwrap() as f32),
                    scale as f32,
                    Vec3A::new(rotation[0].as_f64().unwrap() as f32, rotation[1].as_f64().unwrap() as f32, rotation[2].as_f64().unwrap() as f32),
                    filename,
                    material
                )
            )
        },
        _ => { panic!("Unknown object type: {}", objtype); }
    }
}
