// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Color struct

use glam;
use glam::{vec3a, Vec3A};
pub type Color = Vec3A;


pub fn to_rgb(pixel_color: Color, samples_per_pixel: u32) -> image::Rgb<u8> {
    let scale: f32 = 1.0 / (samples_per_pixel as f32);
    let rgb: Color = (pixel_color * scale).powf(0.5).clamp(vec3a(0.0, 0.0, 0.0), vec3a(0.999, 0.999, 0.999));
    image::Rgb([
        (256.0 * rgb.x) as u8,
        (256.0 * rgb.y) as u8,
        (256.0 * rgb.z) as u8,
    ])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_rgb() -> Result<(), std::fmt::Error> {
        let c: Color = Color::new(0.5, 1.0, 0.0);
        to_rgb(c, 1);
        Ok(())
    }
}
