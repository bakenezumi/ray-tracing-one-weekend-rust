use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    mp: Box<dyn Material>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Box<dyn Material>) -> XyRect {
        XyRect {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k-r.origin.z) / r.direction.z;
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin.x + t*r.direction.x;
        let y = r.origin.y + t*r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            mat_ptr: &*self.mp,
            t: t,
            u: (x-self.x0)/(self.x1-self.x0),
            v: (y-self.y0)/(self.y1-self.y0),
            front_face: false
        };
        rec.set_face_normal(&r, &Vec3::new(0.0, 0.0, 1.0));
        Some(rec)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
