// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 09/02/2023
// Description: This file implements the parsing of YAML config files

use std::sync::Arc;

use yaml_rust::{YamlLoader, Yaml};

use glam::Vec3A;

use crate::material::DiffuseLight;
use crate::hittable_list::HittableList;
use crate::hittable_list::Hittable;
use crate::sphere::Sphere;
use crate::texture::{Texture, SolidColor, ChessBoard, ImageTexture};
use crate::rectangle::{XYRectangle, XZRectangle, YZRectangle};
use crate::bbox::BBox;
use crate::mesh::Mesh;
use crate::material::{Material, Lambertian, Metal, Dielectric, Plastic, GGXGlossy};
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
        let min_depth = hashconsts[&yaml_rust::Yaml::String("minDepth".to_string())].as_i64().unwrap() as u32;
        let environment_map = {
            if hashconsts.contains_key(&yaml_rust::Yaml::String("environmentMap".to_string())) {
                Some(hashconsts[&yaml_rust::Yaml::String("environmentMap".to_string())].as_str().unwrap().to_string())
            } else { None }
        };
        let environment_distance = {
            if hashconsts.contains_key(&yaml_rust::Yaml::String("environmentDistance".to_string())) {
                Some(hashconsts[&yaml_rust::Yaml::String("environmentDistance".to_string())].as_f64().unwrap() as f32)
            } else { None }
        };
        let environment_intensity = {
            if hashconsts.contains_key(&yaml_rust::Yaml::String("environmentIntensity".to_string())) {
                Some(hashconsts[&yaml_rust::Yaml::String("environmentIntensity".to_string())].as_f64().unwrap() as f32)
            } else { None }
        };
        let filter: Option<String> = {
            if hashconsts.contains_key(&yaml_rust::Yaml::String("filter".to_string())) {
                Some(hashconsts[&yaml_rust::Yaml::String("filter".to_string())].as_str().unwrap().to_string())
            } else { None }
        };
        let aspect_ratio = width as f32 / height as f32;
        let sources_lambda: f32 = {
            if hashconsts.contains_key(&yaml_rust::Yaml::String("sourcesLambda".to_string())) {
                hashconsts[&yaml_rust::Yaml::String("sourcesLambda".to_string())].as_f64().unwrap() as f32
            } else { 299792458.0 / 2.45e9 }
        };
        let power_render_center: Vec3A = {
            if hashconsts.contains_key(&yaml_rust::Yaml::String("powerRenderCenter".to_string())) {
                let pow_center = hashconsts[&yaml_rust::Yaml::String("powerRenderCenter".to_string())].as_vec().unwrap();
                Vec3A::new(pow_center[0].as_f64().unwrap() as f32, pow_center[1].as_f64().unwrap() as f32, pow_center[2].as_f64().unwrap() as f32)
            } else { Vec3A::new(0.0, 0.0, 0.0) }
        };
        utility::Constants { width, height, aspect_ratio, samples_per_pixel, max_depth, min_depth, environment_map, environment_distance, environment_intensity, filter, sources_lambda, power_render_center }
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
            let obj: Arc<dyn Hittable + Send + Sync> = _parse_geometry(hashobj, material);
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
            world.push(Arc::new(spherearray));
        } else { panic!("Unsupported object type: {}", objtype) }
    }
    world
}

fn _parse_material(hashobj: &yaml_rust::yaml::Hash) -> Box<dyn Material + Send + Sync> {
    let objmat = hashobj[&yaml_rust::Yaml::String("material".to_string())].as_hash().unwrap();
    let objmattype = objmat[&yaml_rust::Yaml::String("matType".to_string())].as_str().unwrap();
    match objmattype {
        "Lambertian" => { Box::new(Lambertian::new_texture(_parse_texture(objmat))) },
        "Metal" => {
            // has an albedo and a fuzz
            let fuzz = objmat[&yaml_rust::Yaml::String("fuzz".to_string())].as_f64().unwrap();
            Box::new(Metal::new_texture(_parse_texture(objmat), fuzz as f32))
        },
        "Dielectric" => {
            // has just an index of refraction
            let ior = objmat[&yaml_rust::Yaml::String("refractionIdx".to_string())].as_f64().unwrap();
            Box::new(Dielectric::new_texture(_parse_texture(objmat), ior as f32))
        },
        "Plastic" => {
            // has an albedo, a reflectivity and a fuzz
            let fuzz = objmat[&yaml_rust::Yaml::String("fuzz".to_string())].as_f64().unwrap();
            let reflectivity = objmat[&yaml_rust::Yaml::String("reflectivity".to_string())].as_f64().unwrap();
            Box::new(Plastic::new_texture(_parse_texture(objmat), reflectivity as f32, fuzz as f32))
        },
        "GGX" => {
            // has an albedo, a roughness and a fuzz
            let reflectivity = objmat[&yaml_rust::Yaml::String("reflectivity".to_string())].as_f64().unwrap();
            let roughness = objmat[&yaml_rust::Yaml::String("roughness".to_string())].as_f64().unwrap();
            Box::new(GGXGlossy::new_texture(_parse_texture(objmat), roughness as f32, reflectivity as f32))
        }
        "DiffuseLight" => {
            // has just an emittance
            let intensity = objmat[&yaml_rust::Yaml::String("intensity".to_string())].as_f64().unwrap();
            Box::new(DiffuseLight::new_texture(_parse_texture(objmat), intensity as f32))
        },
        _ => { panic!("Unknown material type: {:?}", objmat); }
    }
}

fn _parse_texture(objmat: &yaml_rust::yaml::Hash) -> Box<dyn Texture + Send + Sync> {
    let textype = objmat[&yaml_rust::Yaml::String("texType".to_string())].as_str().unwrap();
    let hashtex = objmat[&yaml_rust::Yaml::String("texture".to_string())].as_hash().unwrap();
    match textype {
        "SolidColor" => {
            // albedo is inside of the hash of SolidColor
            let albedo = hashtex[&yaml_rust::Yaml::String("albedo".to_string())].as_vec().unwrap();
            Box::new(SolidColor::new(Color::new(albedo[0].as_f64().unwrap() as f32, albedo[1].as_f64().unwrap() as f32, albedo[2].as_f64().unwrap() as f32)))
        },
        "ChessBoard" => {
            // Contains two textures and a scale
            let tex1 = _parse_texture(hashtex[&yaml_rust::Yaml::String("tex1".to_string())].as_hash().unwrap());
            let tex2 = _parse_texture(hashtex[&yaml_rust::Yaml::String("tex2".to_string())].as_hash().unwrap());
            let scale = hashtex[&yaml_rust::Yaml::String("scale".to_string())].as_f64().unwrap();
            Box::new(ChessBoard::new(tex1, tex2, scale as f32))
        },
        "ImageTexture" => {
            let filename = hashtex[&yaml_rust::Yaml::String("filename".to_string())].as_str().unwrap();
            Box::new(ImageTexture::new(filename))
        }
        _ => { panic!("Unsupported texture type: {}", textype) }
    }
}

fn _parse_geometry(hashobj: &yaml_rust::yaml::Hash, material: Box<dyn Material>) -> Arc<dyn Hittable + Send + Sync> {
    let objtype = hashobj[&yaml_rust::Yaml::String("objType".to_string())].as_str().unwrap();
    match objtype {
        "Sphere" => {
            // has a center and radius
            let center = hashobj[&yaml_rust::Yaml::String("center".to_string())].as_vec().unwrap();
            let radius = hashobj[&yaml_rust::Yaml::String("radius".to_string())].as_f64().unwrap();
            Arc::new(Sphere::new(Point3::new(center[0].as_f64().unwrap() as f32, center[1].as_f64().unwrap() as f32, center[2].as_f64().unwrap() as f32), radius as f32, material, 0))
        },
        "XYRectangle" => {
            // has a position, width and height
            let position = hashobj[&yaml_rust::Yaml::String("position".to_string())].as_vec().unwrap();
            let width = hashobj[&yaml_rust::Yaml::String("width".to_string())].as_f64().unwrap();
            let height = hashobj[&yaml_rust::Yaml::String("height".to_string())].as_f64().unwrap();
            Arc::new(XYRectangle::new(
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
            Arc::new(XZRectangle::new(
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
            Arc::new(YZRectangle::new(
                position[1].as_f64().unwrap() as f32 - width as f32 / 2.0,
                position[1].as_f64().unwrap() as f32 + width as f32 / 2.0,
                position[2].as_f64().unwrap() as f32 - height as f32 / 2.0,
                position[2].as_f64().unwrap() as f32 + height as f32 / 2.0,
                position[0].as_f64().unwrap() as f32,
                material,
                0
            ))
        },
        "Box" => {
            // has a position, width and height and depth
            let position = hashobj[&yaml_rust::Yaml::String("position".to_string())].as_vec().unwrap();
            let width = hashobj[&yaml_rust::Yaml::String("width".to_string())].as_f64().unwrap();
            let height = hashobj[&yaml_rust::Yaml::String("height".to_string())].as_f64().unwrap();
            let depth = hashobj[&yaml_rust::Yaml::String("depth".to_string())].as_f64().unwrap();
            Arc::new(BBox::new(
                Vec3A::new(position[0].as_f64().unwrap() as f32, position[1].as_f64().unwrap() as f32, position[2].as_f64().unwrap() as f32),
                Vec3A::new(width as f32, height as f32, depth as f32),
                material
            ))
        }
        "Mesh" => {
            // has a filename, position, rotation and scale
            let filename = hashobj[&yaml_rust::Yaml::String("filename".to_string())].as_str().unwrap();
            let position = hashobj[&yaml_rust::Yaml::String("position".to_string())].as_vec().unwrap();
            let rotation = hashobj[&yaml_rust::Yaml::String("rotation".to_string())].as_vec().unwrap();
            let scale = hashobj[&yaml_rust::Yaml::String("scalingFactor".to_string())].as_f64().unwrap();
            Arc::new(
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
