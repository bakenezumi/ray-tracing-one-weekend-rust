use rand::Rng;
use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::vec3::Color;


pub trait Material: Sync {
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
    let scattered = Ray::new(rec.p, reflected+Vec3::random_in_unit_sphere(rng)*self.fuzz, r_in.time);
    let attenuation = self.albedo;
    if scattered.direction.dot(&rec.normal) > 0.0 {
      Some((attenuation, scattered))
    } else {
      None
    }
  }
}

pub struct Dielactric {
  ref_idx: f64
}

impl Dielactric {
  pub fn new(ref_idx: f64) -> Dielactric {
    Dielactric {
      ref_idx
    }
  }
}

impl Material for Dielactric {
  fn scatter<'a>(&self, rng: &'a mut ThreadRng, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
    let attenuation = Vec3::new(1.0, 1.0, 1.0);
    let etai_over_etat = if rec.front_face {
      1.0 / self.ref_idx
    } else {
      self.ref_idx
    };

    let unit_direction = Vec3::unit_vector(&r_in.direction);

    let cos_theta = std::cmp::min_by(-unit_direction.dot(&rec.normal), 1.0, |a, b| a.partial_cmp(b).unwrap());
    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
    if etai_over_etat * sin_theta > 1.0 {
      let reflected = unit_direction.reflect(&rec.normal);
      let scattered = Ray::new(rec.p, reflected, 0.0);
      return Some((attenuation, scattered))
    }
    let reflect_prob = schlick(cos_theta, etai_over_etat);
    if rng.gen::<f64>() < reflect_prob {
      let reflected = unit_direction.reflect(&rec.normal);
      let scattered = Ray::new(rec.p, reflected, 0.0);
      return Some((attenuation, scattered));
    }
    let refracted = unit_direction.refract(&rec.normal, etai_over_etat);
    let scattered = Ray::new(rec.p, refracted, 0.0);
    Some((attenuation, scattered))
  }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0*r0;
  r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
}