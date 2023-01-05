use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

struct HittableList<'a> {
  objects: Vec<&'a dyn Hittable>
}

impl<'a> HittableList<'a> {
  fn clear(&mut self) {
    self.objects.clear();
  }

  fn add(&mut self, object: &'a dyn Hittable) {
    self.objects.push(object)
  }
}

impl Hittable for HittableList<'_> {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> Option<HitRecord> {
    let mut temp_rec: Option<HitRecord> = None;
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    let objects = &(self.objects);
    for object in objects.into_iter() {
      match object.hit(r, t_min, closest_so_far, rec) {
        None => {}
        Some(hit_rec) => {
          hit_anything = true;
          closest_so_far = hit_rec.t;
          temp_rec = Some(hit_rec);
        }
      }
    }
    temp_rec
  }
}
