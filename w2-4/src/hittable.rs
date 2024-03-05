use crate::aabb::Aabb;
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

pub trait CloneHittable {
  fn clone_box(&self) -> Box<dyn Hittable>;
}
pub trait Hittable: Sync + CloneHittable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb>;

}


impl<T> CloneHittable for T
  where
      T: 'static + Hittable + Clone,
{
  fn clone_box(&self) -> Box<dyn Hittable> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn Hittable> {
  fn clone(&self) -> Box<dyn Hittable> {
    self.clone_box()
  }
}