use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct XyRect {
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

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

#[derive(Clone)]
pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Box<dyn Material>,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Box<dyn Material>) -> XzRect {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
            let t = (self.k - r.origin.y) / r.direction.y;
            if t < t0 || t > t1 {
                return None;
            }
            let x = r.origin.x + t * r.direction.x;
            let z = r.origin.z + t * r.direction.z;
            if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
                return None
            }
            let mut rec = HitRecord {
                p: r.at(t),
                normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                mat_ptr: &*self.mp,
                t: t,
                u: (x - self.x0) / (self.x1 - self.x0),
                v: (z - self.z0) / (self.z1 - self.z0),
                front_face: false
            };
            rec.set_face_normal(&r, &Vec3::new(0.0, 1.0, 0.0));
            Some(rec)
        }
    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

#[derive(Clone)]
pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    mp: Box<dyn Material>,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Box<dyn Material>) -> YzRect {
        YzRect {
            y0,
            y1,
            z0,
            z1,
            k,
            mp,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t0 || t > t1 {
            return None;
        }
        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None
        }
        let mut rec = HitRecord {
            p: r.at(t),
            normal: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            mat_ptr: &*self.mp,
            t: t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: false
        };
        rec.set_face_normal(&r, &Vec3::new(1.0, 0.0, 0.0));
        Some(rec)
    }
    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
