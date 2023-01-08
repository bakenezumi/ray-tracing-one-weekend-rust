use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;

pub struct Camera {
  origin: Point3,
  lower_left_corner: Point3,
  horizontal: Vec3,
  vertical: Vec3
}

impl Camera {
  pub fn new() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_heght = 2.0;
    let viewport_width = aspect_ratio * viewport_heght;
    let focal_length = 1.0;

    let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x:0.0, y: viewport_heght, z: 0.0 };
    Camera {
      origin: origin,
      horizontal: horizontal,
      vertical: vertical,
      lower_left_corner: {
        origin - horizontal/2.0 - vertical/2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length }
      }
    }
  }

  pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    Ray {
      origin: self.origin,
      direction: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin
    }
  }
}
