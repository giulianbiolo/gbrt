// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements various utility constants and functions used throughout the project
use rand::prelude::*;

// Image constants
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = ((WIDTH as f32) / ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 16; // Antialiasing samples per pixel

// Camera constants
pub const VIEWPORT_HEIGHT: f32 = 2.0;
pub const VIEWPORT_WIDTH: f32 = ASPECT_RATIO as f32 * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f32 = 1.0;

// Numerical Constants
pub const INFINITY: f32 = std::f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

// Utility functions
pub fn degrees_to_radians(degrees: f32) -> f32 { degrees * PI / 180.0 }
pub fn random_f32() -> f32 { rand::random::<f32>() }
pub fn random_f32_range(min: f32, max: f32) -> f32 { min + (max - min) * random_f32() }
pub fn clamp(x: f32, min: f32, max: f32) -> f32 { if x < min { min } else if x > max { max } else { x } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_degrees_to_radians() -> Result<(), std::fmt::Error> {
        assert_eq!(degrees_to_radians(180.0), PI);
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
        assert_eq!(clamp(0.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp(1.0, 0.0, 1.0), 1.0);
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(2.0, 0.0, 1.0), 1.0);
        Ok(())
    }
}
