// Author: Giulian Biolo, github.com/giulianbiolo
// Date: 24/01/2023
// Description: This file implements the Color struct

use glam::Vec3A;
pub type Color = Vec3A;


pub fn to_rgb(pixel_color: Color, samples_per_pixel: f32) -> image::Rgb<u8> {
    let mut pixel_color: Vec3A = pixel_color;
    if pixel_color.x.is_nan() { pixel_color.x = 0.0; }
    if pixel_color.y.is_nan() { pixel_color.y = 0.0; }
    if pixel_color.z.is_nan() { pixel_color.z = 0.0; }
    let scale: f32 = 1.0 / samples_per_pixel;
    let rgb: Color = (pixel_color * scale).powf(0.5).clamp(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(0.999, 0.999, 0.999));
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
        to_rgb(c, 1.0);
        Ok(())
    }
}
