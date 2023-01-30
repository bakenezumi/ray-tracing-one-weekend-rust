use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

pub struct HittableList {
  objects: Vec<Box<dyn Hittable>>
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
}
