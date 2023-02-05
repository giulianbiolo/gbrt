// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various utility constants and functions used throughout the project
use glam;
use glam::{vec3a, Vec3A};



// Image constants
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const ASPECT_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);
// pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
// pub const HEIGHT: u32 = ((WIDTH as f32) / ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 32; // Antialiasing samples per pixel
pub const MAX_DEPTH: u32 = 50; // Maximum ray bounces

// Numerical Constants
pub const INFINITY: f32 = std::f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;
pub const EPSILON: f32 = 0.0001;

// Utility functions
pub fn random_f32() -> f32 { rand::random::<f32>() }
pub fn random_f32_range(min: f32, max: f32) -> f32 { min + (max - min) * random_f32() }

pub fn random_in_unit_disk() -> Vec3A {
    loop {
        let p: Vec3A = vec3a(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), 0.0);
        if p.dot(p) < 1.0 { return p; }
    }
}

pub fn random_unit_vector() -> Vec3A {
    let a: f32 = random_f32_range(0.0, 2.0 * PI);
    let z: f32 = random_f32_range(-1.0, 1.0);
    let r: f32 = (1.0 - z * z).sqrt();
    vec3a(r * a.cos(), r * a.sin(), z)
}

pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p: Vec3A = vec3a(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0));
        if p.dot(p) < 1.0 { return p; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_degrees_to_radians() -> Result<(), std::fmt::Error> {
        assert_eq!(f32::to_radians(180.0), PI);
        Ok(())
    }
    #[test]
    fn test_random_f32() -> Result<(), std::fmt::Error> {
        let r = random_f32();
        assert!(r >= 0.0 && r < 1.0);
        Ok(())
    }
    #[test]
    fn test_random_f32_range() -> Result<(), std::fmt::Error> {
        let r = random_f32_range(0.0, 1.0);
        assert!(r >= 0.0 && r < 1.0);
        Ok(())
    }
    #[test]
    fn test_clamp() -> Result<(), std::fmt::Error> {
        assert_eq!(Vec3A::new(-1.0, 0.0, 2.0).clamp(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0)), Vec3A::new(0.0, 0.0, 1.0));
        Ok(())
    }
}
