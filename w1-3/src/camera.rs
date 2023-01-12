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
  pub fn new(
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64
  ) -> Camera {
    let theta = vfov.to_radians();
    let h = (theta/2.0).tan();
    let viewport_heght = 2.0 - h;
    let viewport_width = aspect_ratio * viewport_heght;

    let w = (lookfrom - lookat).unit_vector();
    let u = vup.cross(&w).unit_vector();
    let v = w.cross(&u);

    let origin = lookfrom;
    let horizontal = u * viewport_width;
    let vertical = v * viewport_heght;

    Camera {
      origin: origin,
      horizontal: horizontal,
      vertical: vertical,
      lower_left_corner: {
        origin - horizontal/2.0 - vertical/2.0 - w
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
