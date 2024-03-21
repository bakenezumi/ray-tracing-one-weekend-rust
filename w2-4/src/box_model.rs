use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::rect::{XyRect, XzRect, YzRect};
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct BoxModel {
  pub(crate) box_min: Vec3,
  pub(crate) box_max: Vec3,
  pub(crate) sides: HittableList,
}

impl BoxModel {
  pub fn new(p0: Vec3, p1: Vec3, ptr: Box<dyn Material>) -> BoxModel {
    let mut sides = HittableList::new();
    sides.add(Box::new(XyRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone_box())));
    sides.add(Box::new(XyRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, ptr.clone_box())));
    sides.add(Box::new(XzRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone_box())));
    sides.add(Box::new(XzRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, ptr.clone_box())));
    sides.add(Box::new(YzRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone_box())));
    sides.add(Box::new(YzRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr.clone_box())));

    BoxModel {
      box_min: p0,
      box_max: p1,
      sides
    }
  }
}

impl Hittable for BoxModel {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    self.sides.hit(r, t_min, t_max)
  }

  fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
    Some(Aabb::new(self.box_min, self.box_max))
  }
}