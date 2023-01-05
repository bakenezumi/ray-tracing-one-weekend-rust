use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Point3;

struct Sphere {
  center: Point3,
  radius: f64
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - self.radius - self.radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant > 0.0 {
      let root = discriminant.sqrt();
      let temp = (-half_b - root)/a;
      if temp < t_max && temp > t_min {
        let p = r.at(rec.t);
        let mut hit_rec = HitRecord {
          p: r.at(rec.t),
          normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
          t: temp,
          front_face: false
        };
        let outward_normal = (hit_rec.p - self.center) / self.radius;
        hit_rec.set_face_normal(r, &outward_normal);
        return Some(hit_rec);
      }
      let temp2 = (-half_b + root) / a;
      if temp2 < t_max && temp2 > t_min {
        let p = r.at(rec.t);
        let mut hit_rec = HitRecord {
          p: r.at(rec.t),
          normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
          t: temp2,
          front_face: false
        };
        let outward_normal = (hit_rec.p - self.center) / self.radius;
        hit_rec.set_face_normal(r, &outward_normal);
        return Some(hit_rec);
      }
    }
    None
  }
}
