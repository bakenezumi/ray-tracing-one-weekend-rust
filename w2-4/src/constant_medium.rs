use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;


#[derive(Clone)]
struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Box<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Box<dyn Hittable>, d: f64, a: Box<dyn Material>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: a,
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec1 = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY);
        if rec1.is_none() {
            return None;
        }
        let rec1 = rec1?;

        let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY);
        if rec2.is_none() {
            return None;
        }
        let rec2 = rec2?;

        let rec1t = if rec1.t < t_min { t_min } else { rec1.t };
        let rec2t = if rec2.t > t_max { t_max } else { rec2.t };

        if rec1t >= rec2t {
            return None;
        }

        let rec1t = if rec1t < 0.0 { 0.0 } else { rec1t };

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2t - rec1t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        Some(HitRecord {
            p: r.at(rec1t + hit_distance / ray_length),
            normal: Vec3::new(1.0, 0.0, 0.0),
            mat_ptr: &*self.phase_function,
            t: rec1t + hit_distance / ray_length,
            u: 0.0,
            v: 0.0,
            front_face: true,
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.boundary.bounding_box(t0, t1)
    }
}

