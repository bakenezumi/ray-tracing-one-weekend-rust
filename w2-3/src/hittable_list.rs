use crate::aabb::{Aabb, surrounding_box};
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

#[derive(Clone)]
pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
  pub fn new() -> HittableList {
    HittableList { objects: Vec::new() }
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }

  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object)
  }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut temp_rec: Option<HitRecord> = None;
    let mut closest_so_far = t_max;
    let objects = &(self.objects);
    for object in objects.into_iter() {
      match object.hit(r, t_min, closest_so_far) {
        None => {}
        Some(hit_rec) => {
          closest_so_far = hit_rec.t;
          temp_rec = Some(hit_rec);
        }
      }
    }
    temp_rec
  }

  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
    if self.objects.is_empty() { return None };
    let mut output_box: Option<Aabb> = None;

    for object in <Vec<Box<dyn Hittable>> as Clone>::clone(&self.objects).into_iter() {
      match object.bounding_box(t0, t1) {
        None => return None,
        Some(temp_box) => {
          output_box = match output_box {
            None => Some(temp_box),
            Some(x) => Some(surrounding_box(x, temp_box))
          }
        },
      }
    }
    output_box
  }
}
