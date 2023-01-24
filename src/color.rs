// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Color struct
use crate::vec3;
use crate::utility;

impl vec3::Color {
    pub fn rgb_from(pixel_color: vec3::Color) -> image::Rgb<u8> {
        // Write the translated [0,255] value of each color component.
        image::Rgb([
            (255.999 * pixel_color.x()) as u8,
            (255.999 * pixel_color.y()) as u8,
            (255.999 * pixel_color.z()) as u8,
        ])
    }
    pub fn to_rgb(&self, samples_per_pixel: u32) -> image::Rgb<u8> {
        // Write the translated [0,255] value of each color component.
        let scale: f32 = 1.0 / (samples_per_pixel as f32);
        let r: f32 = scale * self.x();
        let g: f32 = scale * self.y();
        let b: f32 = scale * self.z();
        image::Rgb([
            (256.0 * utility::clamp(r, 0.0, 0.999) as f32) as u8,
            (256.0 * utility::clamp(g, 0.0, 0.999) as f32) as u8,
            (256.0 * utility::clamp(b, 0.0, 0.999) as f32) as u8,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rgb_from() -> Result<(), std::fmt::Error> {
        let c = vec3::Color::new(0.5, 0.0, 0.0);
        let rgb = vec3::Color::rgb_from(c);
        assert_eq!(rgb, image::Rgb([127, 0, 0]));
        Ok(())
    }
    #[test]
    fn test_to_rgb() -> Result<(), std::fmt::Error> {
        let c = vec3::Color::new(0.5, 1.0, 0.0);
        let rgb = c.to_rgb(1);
        assert_eq!(rgb, image::Rgb([128, 255, 0]));
        Ok(())
    }
}
