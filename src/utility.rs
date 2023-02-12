// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various utility constants and functions used throughout the project

use std::env::args;

use lazy_static::lazy_static;

use glam;
use glam::Vec3A;

use crate::parser;


#[derive(Debug, Clone, Copy)]
pub struct Constants {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub background: Vec3A,
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            aspect_ratio: 800.0 / 600.0,
            samples_per_pixel: 64,
            max_depth: 5000,
            background: Vec3A::new(0.7, 0.7, 0.7),
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

// Utility functions
pub fn random_f32() -> f32 { rand::random::<f32>() }
pub fn random_f32_range(min: f32, max: f32) -> f32 { min + (max - min) * random_f32() }

pub fn random_in_unit_disk() -> Vec3A {
    loop {
        let p: Vec3A = Vec3A::new(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), 0.0);
        if p.dot(p) < 1.0 { return p; }
    }
}

pub fn random_unit_vector() -> Vec3A {
    let a: f32 = random_f32_range(0.0, 2.0 * PI);
    let z: f32 = random_f32_range(-1.0, 1.0);
    let r: f32 = (1.0 - z * z).sqrt();
    Vec3A::new(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p: Vec3A = Vec3A::new(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0));
        if p.dot(p) < 1.0 { return p; }
    }
}

pub fn random_in_hemisphere(normal: &Vec3A) -> Vec3A {
    let in_unit_sphere: Vec3A = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 { in_unit_sphere } else { -in_unit_sphere }
}

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
