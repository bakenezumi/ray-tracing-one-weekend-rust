use rand::rngs::ThreadRng;

use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::vec3::Color;
use crate::material::Material;

pub struct Sphere {
  pub center: Point3,
  pub radius: f64,
  pub mat_ptr: Box<dyn Material>
}

impl Sphere {
  pub fn new(cen: Point3, r: f64, m: Box<dyn Material>) -> Sphere {
    Sphere {
      center: cen,
      radius: r,
      mat_ptr: m
    }
  }
}

impl Hittable for Sphere {

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
          mat_ptr: &*self.mat_ptr,
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
          mat_ptr: &*self.mat_ptr,
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
    let scattered = Ray::new(rec.p, scatter_direction, 0.0);
    let attenuation = self.albedo;
    Some((attenuation, scattered))
  }
}

pub struct MovingSphere {
  pub center0: Point3,
  pub center1: Point3,
  pub time0: f64,
  pub time1: f64,
  pub radius: f64,
  pub mat_ptr: Box<dyn Material>
}

impl MovingSphere {
  pub fn new(cen0: Point3, cen1: Point3, t0: f64, t1: f64, r: f64, m: Box<dyn Material>) -> MovingSphere {
    MovingSphere {
      center0: cen0,
      center1: cen1,
      time0: t0,
      time1: t1,
      radius: r,
      mat_ptr: m
    }
  }
  pub fn center(&self, time: f64) -> Point3 {
    self.center0 + ((time - self.time0) / (self.time1 - self.time0))*(self.center1 - self.center0)
  }
}

impl Hittable for MovingSphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = r.origin - self.center(r.time);
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
          mat_ptr: &*self.mat_ptr,
          t: temp,
          front_face: false
        };
        let outward_normal = (hit_rec.p - self.center(r.time)) / self.radius;
        hit_rec.set_face_normal(r, &outward_normal);
        return Some(hit_rec);
      }
      let temp2 = (-half_b + root) / a;
      if temp2 < t_max && temp2 > t_min {
        let mut hit_rec = HitRecord {
          p: r.at(temp2),
          normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
          mat_ptr: &*self.mat_ptr,
          t: temp2,
          front_face: false
        };
        let outward_normal = (hit_rec.p - self.center(r.time)) / self.radius;
        hit_rec.set_face_normal(r, &outward_normal);
        return Some(hit_rec);
      }
    }
    None
  }
}
