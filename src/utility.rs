// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various utility constants and functions used throughout the project

use std::env::args;
use lazy_static::lazy_static;

use glam::Vec3A;

use crate::parser;


#[derive(Debug, Clone, Copy)]
pub struct Constants {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    pub samples_per_pixel: u32,
    pub max_depth: u32
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            aspect_ratio: 1920.0 / 1080.0,
            samples_per_pixel: 64,
            max_depth: 5000
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

// Background SkyBox
pub const BLUE_SKY: Vec3A = Vec3A::new(0.5, 0.7, 1.0);

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
