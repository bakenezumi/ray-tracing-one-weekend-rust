use crate::vec3::Vec3;
use crate::vec3::Point3;

pub struct Ray {
  pub origin: Point3,
  pub direction: Vec3
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3) -> Ray {
    Ray {
      origin,
      direction
    }
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.origin + (self.direction * t)
  }
}
