use std::fmt::Debug;
use std::sync::Arc;

use dyn_clone::DynClone;
use image::{GenericImageView, DynamicImage};

use crate::point3::Point3;
use crate::color::Color;


pub trait Texture: DynClone + Debug + Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

dyn_clone::clone_trait_object!(Texture);

/****************** Solid Color ******************/
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> SolidColor { SolidColor { color_value } }
    pub fn from_rgb(r: f32, g: f32, b: f32) -> SolidColor { SolidColor { color_value: Color::new(r, g, b) } }
}

impl Debug for SolidColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolidColor")
            .field("color_value", &self.color_value)
            .finish()
    }
}

impl Clone for SolidColor { fn clone(&self) -> Self { SolidColor { color_value: self.color_value } } }

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Color { self.color_value }
}

/****************** Chess Board ******************/
pub struct ChessBoard {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    scale: f32
}

impl ChessBoard {
    pub fn new_from_colors(odd: Color, even: Color, scale: f32) -> ChessBoard { ChessBoard { odd: Box::new(SolidColor::new(odd)), even: Box::new(SolidColor::new(even)), scale } }
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>, scale: f32) -> ChessBoard { ChessBoard { odd, even, scale } }
}

impl Debug for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChessBoard")
            .field("odd", &self.odd)
            .field("even", &self.even)
            .finish()
    }
}

impl Clone for ChessBoard { fn clone(&self) -> Self { ChessBoard { odd: self.odd.clone(), even: self.even.clone(), scale: self.scale.clone() } } }

impl Texture for ChessBoard {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let sines: f32 = (self.scale * p.x).sin() * (self.scale * p.y).sin() * (self.scale * p.z).sin();
        if sines < 0.0 { self.odd.value(u, v, p) } else { self.even.value(u, v, p) }
    }
}

/****************** Gradient Color ******************/
pub struct GradientColor {
    top: Box<dyn Texture>,
    bottom: Box<dyn Texture>,
}

impl GradientColor {
    pub fn new(top: Box<dyn Texture>, bottom: Box<dyn Texture>) -> GradientColor { GradientColor { top, bottom } }
}

impl Debug for GradientColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GradientColor")
            .field("top", &self.top)
            .field("bottom", &self.bottom)
            .finish()
    }
}

impl Clone for GradientColor { fn clone(&self) -> Self { GradientColor { top: self.top.clone(), bottom: self.bottom.clone() } } }

impl Texture for GradientColor {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let t = 0.5 * (p.y + 1.0);
        (1.0 - t) * self.bottom.value(u, v, p) + t * self.top.value(u, v, p)
    }
}

/****************** Image Texture ******************/
pub struct ImageTexture {
    image: Arc<DynamicImage>,
    width: u32,
    height: u32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        println!("Loading image texture from file: {}", filename);
        let image = image::open(filename).unwrap();
        let (width, height) = image.dimensions();
        ImageTexture { image: Arc::new(image), width, height }
    }
}

impl Debug for ImageTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageTexture")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Clone for ImageTexture {
    fn clone(&self) -> Self {
        ImageTexture {
            image: self.image.clone(),
            width: self.width,
            height: self.height
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3) -> Color {
        //println!("Input: u={}, v={}", u, v);
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let i = ((u * self.width as f32) as u32).min(self.width - 1);
        let j = ((v * self.height as f32) as u32).min(self.height - 1);
        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);
        //println!("Pixel: {:?} at position [{}, {}]", pixel, i, j);
        Color::new(pixel[0] as f32 * color_scale, pixel[1] as f32 * color_scale, pixel[2] as f32 * color_scale)
    }
}

/****************** Environment Map Texture ******************/
pub struct EnvironmentMapTexture {
    image: Arc<DynamicImage>,
    width: u32,
    height: u32,
}

impl EnvironmentMapTexture {
    pub fn new(filename: &str) -> EnvironmentMapTexture {
        println!("Loading environment map texture from file: {}", filename);
        let image = image::open(filename).unwrap();
        let (width, height) = image.dimensions();
        EnvironmentMapTexture { image: Arc::new(image), width, height }
    }
}

impl Debug for EnvironmentMapTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvironmentMapTexture")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Clone for EnvironmentMapTexture {
    fn clone(&self) -> Self {
        EnvironmentMapTexture {
            image: self.image.clone(),
            width: self.width,
            height: self.height
        }
    }
}

impl Texture for EnvironmentMapTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3) -> Color {
        //println!("Input: u={}, v={}", u, v);
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let i = ((u * self.width as f32) as u32).min(self.width - 1);
        let j = ((v * self.height as f32) as u32).min(self.height - 1);
        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);
        //println!("Pixel: {:?} at position [{}, {}]", pixel, i, j);
        Color::new(pixel[0] as f32 * color_scale, pixel[1] as f32 * color_scale, pixel[2] as f32 * color_scale)
    }
}
