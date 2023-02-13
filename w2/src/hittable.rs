use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::material::Material;

pub struct HitRecord<'a> {
  pub p: Point3,
  pub normal: Vec3,
  pub mat_ptr: &'a dyn Material,
  pub t: f64,
  pub front_face: bool
}

impl HitRecord<'_> {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    self.front_face = r.direction.dot(outward_normal) < 0.0;
    self.normal = if self.front_face {
      *outward_normal
    } else {
      -*outward_normal
    }
  }
}

pub trait Hittable: Sync {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
