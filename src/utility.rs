// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various utility constants and functions used throughout the project

use std::env::args;
use std::sync::Arc;
use lazy_static::lazy_static;
use fastrand;

use glam::Vec3A;

use crate::hittable_list::Hittable;
use crate::material::DiffuseLight;
use crate::parser;
use crate::point3::Point3;
use crate::sphere::Sphere;
use crate::texture::{self, GradientColor, ImageTexture};
use crate::sampling_filters::{Filter, TentFilter, UniformFilter, LanczosFilter};


#[derive(Debug, Clone)]
pub struct Constants {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub min_depth: u32,
    pub environment_map: Option<String>,
    pub environment_distance: Option<f32>,
    pub environment_intensity: Option<f32>,
    pub filter: Option<String>,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            aspect_ratio: 1920.0 / 1080.0,
            samples_per_pixel: 64,
            max_depth: 5000,
            min_depth: 5,
            environment_map: None,
            environment_distance: None,
            environment_intensity: None,
            filter: None,
        }
    }
}

// Image constants
lazy_static! { pub static ref CONSTS: Constants = parser::parse_yaml_constants(&args().nth(1).unwrap_or("".to_string())); }

// Numerical Constants
pub const INFINITY: f32 = std::f32::INFINITY;
pub const NEG_INFINITY: f32 = std::f32::NEG_INFINITY;
pub const PI: f32 = std::f32::consts::PI;
pub const EPSILON: f32 = 0.0001;
pub const NEAR_ZERO: f32 = 0.001;

// Background SkyBox
pub const BLUE_SKY: Vec3A = Vec3A::new(0.5, 0.7, 1.0);

// Utility functions
pub fn load_environment() -> Arc<dyn Hittable + Send + Sync> {
    let env_dist: f32 = if CONSTS.environment_distance.is_some() { CONSTS.environment_distance.unwrap() } else { 1000.0 };
    let env_intensity: f32 = if CONSTS.environment_intensity.is_some() { CONSTS.environment_intensity.unwrap() } else { 1.0 };
    println!("Environment distance: {}", env_dist);
    println!("Environment map: {:?}", CONSTS.environment_map);
    if CONSTS.environment_map.is_some() {
        // environment map is just a textured sphere with a diffuse light material
        let env_tex: ImageTexture = texture::EnvironmentMapTexture::new(CONSTS.environment_map.as_ref().unwrap());
        let env_mat: DiffuseLight = DiffuseLight::new_texture(Box::new(env_tex), env_intensity);
        let env_sphere: Sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), env_dist, Box::new(env_mat), 0);
        Arc::new(env_sphere)
    } else {
        let env_tex: GradientColor = texture::GradientColor::new(
            Box::new(texture::SolidColor::new(BLUE_SKY)),
            Box::new(texture::SolidColor::new(Vec3A::ONE))
        );
        // Box::new(Sphere::new(Vec3A::new(0.0, 0.0, 0.0), env_dist, Box::new(DiffuseLight::new_texture(Box::new(env_tex), 1.0)), 0))
        Arc::new(Sphere::new(Vec3A::new(0.0, 0.0, 0.0), env_dist, Box::new(DiffuseLight::new_texture(Box::new(env_tex), env_intensity)), 0))
    }
}
pub fn load_filter() -> Box<dyn Filter + Send + Sync> {
    if CONSTS.filter.is_some() {
        match CONSTS.filter.as_ref().unwrap().as_str() {
            "UniformFilter" => Box::new(UniformFilter::new()),
            "TentFilter" => Box::new(TentFilter::new()),
            "LanczosFilter" => Box::new(LanczosFilter::new()),
            _ => Box::new(UniformFilter::new())
        }
    } else { Box::new(UniformFilter::new()) }
}

pub fn random_f32() -> f32 { fastrand::f32() }
pub fn random_f32_range(min: f32, max: f32) -> f32 { fastrand::f32() * (max - min) + min }
pub fn random_usize_range(min: usize, max: usize) -> usize { fastrand::usize(min..max) }

pub fn random_in_unit_disk() -> Vec3A {
    let mut p: Vec3A;
    loop {
        p = Vec3A::new(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 { return p; }
    }
}

pub fn random_unit_vector() -> Vec3A {
    let a: f32 = random_f32_range(0.0, 2.0 * PI);
    let z: f32 = random_f32_range(-1.0, 1.0);
    let r: f32 = (1.0 - z * z).sqrt();
    Vec3A::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_unit_sphere() -> Vec3A {
    let mut p: Vec3A;
    loop {
        p = Vec3A::new(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0));
        if p.length_squared() < 1.0 { return p; }
    }
}
/*
pub fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3A {
    let r1: f32 = random_f32();
    let r2: f32 = random_f32();
    let z: f32 = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);
    let phi: f32 = 2.0 * PI * r1;
    let x: f32 = phi.cos() * (1.0 - z * z).sqrt();
    let y: f32 = phi.sin() * (1.0 - z * z).sqrt();
    Vec3A::new(x, y, z)
}
*/

pub fn random_cosine_direction() -> Vec3A {
    let r1: f32 = random_f32();
    let r2: f32 = random_f32();
    let z: f32 = (1.0 - r2).sqrt();
    let phi: f32 = 2.0 * PI * r1;
    let x: f32 = phi.cos() * r2.sqrt();
    let y: f32 = phi.sin() * r2.sqrt();
    Vec3A::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_f32() -> Result<(), std::fmt::Error> {
        let r: f32 = random_f32();
        assert!(r >= 0.0 && r < 1.0);
        Ok(())
    }
    #[test]
    fn test_random_f32_range() -> Result<(), std::fmt::Error> {
        let r: f32 = random_f32_range(0.0, 1.0);
        assert!(r >= 0.0 && r < 1.0);
        Ok(())
    }
    #[test]
    fn test_random_in_unit_disk() -> Result<(), std::fmt::Error> {
        let r: Vec3A = random_in_unit_disk();
        assert!(r.dot(r) < 1.0);
        Ok(())
    }
    #[test]
    fn test_random_unit_vector() -> Result<(), std::fmt::Error> {
        let r: Vec3A = random_unit_vector();
        assert!(r.length() - 1.0 <= EPSILON);
        Ok(())
    }
    #[test]
    fn test_random_in_unit_sphere() -> Result<(), std::fmt::Error> {
        let r: Vec3A = random_in_unit_sphere();
        assert!(r.dot(r) < 1.0);
        Ok(())
    }
}
