#[path = "ray.rs"] pub mod ray;
use ray::Ray;
use ray::vec3::Vec3;
use ray::vec3::Point3;

pub struct HitRecoad {
  pub p: Point3,
  pub normal: Vec3,
  pub t: f64
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecoad) -> bool;
}
