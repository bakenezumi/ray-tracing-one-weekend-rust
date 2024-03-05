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
