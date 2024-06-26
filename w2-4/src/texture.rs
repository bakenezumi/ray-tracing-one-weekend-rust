use image::GenericImageView;
use rand::prelude::ThreadRng;
use crate::perlin::Perlin;
use crate::vec3::{Color, Point3};


pub trait CloneTexture {
    fn clone_box(&self) -> Box<dyn Texture>;
}
pub trait Texture : Sync + CloneTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl<T> CloneTexture for T
    where
        T: 'static + Texture + Clone,
{
    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct SolidColor {
    color_value: Color
}


impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor {
            color_value: color
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: &Point3) -> Color {
        self.color_value
    }
}

#[derive(Clone)]
pub struct CheckerTexture {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture{
            even: even,
            odd: odd
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0*p.x).sin()*(10.0*p.y).sin()*(10.0*p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}
impl NoiseTexture {
    pub fn new(rng: &mut ThreadRng, scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(rng),
            scale
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * ( 1.0 + (self.scale * p.z + 10.0 * self.noise.turb(*p, 7)).sin() )
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<Color>,
    width: u32,
    height: u32
}

impl ImageTexture {
    pub fn new(file_path: &'static str) -> ImageTexture {
        let img = image::open(file_path).unwrap();

        let (width, height) = img.dimensions();

        let rgb = img.into_rgb8();
        let mut data = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let pixel = rgb.get_pixel(x, y);
                data.push(Color::new(
                    pixel[0] as f64 / 255.0,
                    pixel[1] as f64 / 255.0,
                    pixel[2] as f64 / 255.0
                ));
            }
        }

        ImageTexture {
            data,
            width,
            height
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: &Point3) -> Color {
        let u = u.clamp(0.0, 0.9999);
        let v = 1.0 - v.clamp(0.0, 0.9999);

        let i = (u * self.width as f64) as u32;
        let j = (v * self.height as f64) as u32;

        self.data[(i + self.width * j) as usize]
    }
}
