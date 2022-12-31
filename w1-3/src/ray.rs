#[path = "vec3.rs"] pub mod vec3;
use vec3::Vec3;
use vec3::Point3;

pub struct Ray {
  pub origin: Point3,
  pub direction: Vec3
}

impl Ray {
  pub fn at(&self, t: f64) -> Point3 {
    self.origin + (self.direction * t)
  }
}
