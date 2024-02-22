use crate::vec3::Vec3;
use crate::vec3::Point3;

pub struct Ray {
  pub origin: Point3,
  pub direction: Vec3,
  pub time: f64
}

impl Ray {
  pub fn new(origin: Point3, direction: Vec3, time: f64) -> Ray {
    Ray {
      origin,
      direction,
      time
    }
  }

  pub fn at(&self, t: f64) -> Point3 {
    self.origin + (self.direction * t)
  }
}
