use rand::rngs::ThreadRng;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::vec3::Color;
use crate::material::Material;

pub struct Sphere<'a> {
  pub center: Point3,
  pub radius: f64,
  pub mat_ptr: &'a dyn Material
}

impl Sphere<'_> {
  pub fn new(cen: Point3, r: f64, m: &dyn Material) -> Sphere {
    Sphere {
      center: cen,
      radius: r,
      mat_ptr: m
    }
  }
}

impl Hittable for Sphere<'_> {

  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin - self.center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.length_squared() - self.radius * self.radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant > 0.0 {
      let root = discriminant.sqrt();
      let temp = (-half_b - root)/a;
      if temp < t_max && temp > t_min {
        let mut hit_rec = HitRecord {
          p: r.at(temp),
          normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
          mat_ptr: self.mat_ptr,
          t: temp,
          front_face: false
        };
        let outward_normal = (hit_rec.p - self.center) / self.radius;
        hit_rec.set_face_normal(r, &outward_normal);
        return Some(hit_rec);
      }
      let temp2 = (-half_b + root) / a;
      if temp2 < t_max && temp2 > t_min {
        let mut hit_rec = HitRecord {
          p: r.at(temp2),
          normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
          mat_ptr: self.mat_ptr,
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

pub struct Lambertian {
  albedo: Color  
}

impl Lambertian {
  pub fn new(albedo: Color) -> Lambertian {
    Lambertian {
      albedo
    }
  }
}

impl Material for Lambertian {
  fn scatter<'a>(&self, rng: &'a mut ThreadRng, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
    let scattered = Ray::new(rec.p, scatter_direction);
    let attenuation = self.albedo;
    Some((attenuation, scattered))
  }
}
