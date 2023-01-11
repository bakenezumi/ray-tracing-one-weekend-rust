use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::vec3::Color;


pub trait Material {
  fn scatter<'a>(&self, rng: &'a mut ThreadRng, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Metal {
  albedo: Color,
  fuzz: f64
}

impl Metal {
  pub fn new(albedo: Color, fuzz: f64) -> Metal {
    Metal {
      albedo,
      fuzz
    }
  }
}

impl Material for Metal {
  fn scatter<'a>(&self, rng: &'a mut ThreadRng, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
    let scattered = Ray::new(rec.p, reflected+Vec3::random_in_unit_sphere(rng)*self.fuzz);
    let attenuation = self.albedo;
    if scattered.direction.dot(&rec.normal) > 0.0 {
      Some((attenuation, scattered))
    } else {
      None
    }
  }
}
