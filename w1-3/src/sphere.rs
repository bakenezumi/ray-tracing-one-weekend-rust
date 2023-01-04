use crate::hittable::Hittable;
use crate::hittable::HitRecoad;
use crate::ray::Ray;
use crate::vec3::Point3;

struct Sphere {
  center: Point3,
  radius: f64
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecoad) -> bool {
    let oc = r.origin - self.center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - self.radius - self.radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant > 0.0 {
      let root = discriminant.sqrt();
      let temp = (-half_b - root)/a;
      if temp < t_max && temp > t_min {
        rec.t = temp;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        return true;
      }
      let temp2 = (-half_b + root) / a;
      if temp2 < t_max && temp2 > t_min {
        rec.t = temp2;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        return true;
      }
    }
    false
  }
}
