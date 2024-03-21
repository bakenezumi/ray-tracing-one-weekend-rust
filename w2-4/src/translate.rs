use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
struct Translate {
    ptr: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Box<dyn Hittable>, displacement: Vec3) -> Translate {
        Translate {
            ptr: p,
            offset: displacement,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        self.ptr.hit(&moved_r, t_min, t_max).map(|mut rec| {
            let mut rec2 = HitRecord {
                p: rec.p + self.offset,
                normal: rec.normal,
                mat_ptr: rec.mat_ptr,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face: rec.front_face,
            };
            rec2.set_face_normal(&moved_r, &rec.normal);
            rec2
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.ptr.bounding_box(t0, t1).map(|output_box| {
            Aabb::new(
                output_box.min + self.offset,
                output_box.max + self.offset,
            )
        })
    }
}
