use rand::rngs::ThreadRng;

use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;

pub struct Camera {
  origin: Point3,
  lower_left_corner: Point3,
  horizontal: Vec3,
  vertical: Vec3,
  u: Vec3,
  v: Vec3,
  lens_radius: f64
}

impl Camera {
  pub fn new(
    lookfrom: Point3,
    lookat: Point3,
    vup: Vec3,
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64
  ) -> Camera {
    let theta = vfov.to_radians();
    let h = (theta/2.0).tan();
    let viewport_heght = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_heght;

    let w = (lookfrom - lookat).unit_vector();
    let u = vup.cross(&w).unit_vector();
    let v = w.cross(&u);

    let origin = lookfrom;
    let horizontal = u * (focus_dist * viewport_width);
    let vertical = v * (focus_dist * viewport_heght);

    Camera {
      origin: origin,
      horizontal: horizontal,
      vertical: vertical,
      u: u,
      v: v,
      lower_left_corner: {
        origin - horizontal/2.0 - vertical/2.0 - w*focus_dist
      },
      lens_radius: aperture / 2.0
    }
  }

  pub fn get_ray(&self, rng: &mut ThreadRng, s: f64, t: f64) -> Ray {
    let rd = Vec3::random_in_unit_disk(rng) * self.lens_radius;
    let offset = self.u * rd.x + self.v * rd.y;
    Ray {
      origin: self.origin + offset,
      direction: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset
    }
  }
}
