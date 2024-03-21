use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct RotateY {
    pub ptr: Box<dyn Hittable>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(p: Box<dyn Hittable>, angle: f64) -> RotateY {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = p.bounding_box(0.0, 1.0).unwrap();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x + (1 - i) as f64 * bbox.min.x;
                    let y = j as f64 * bbox.max.y + (1 - j) as f64 * bbox.min.y;
                    let z = k as f64 * bbox.max.z + (1 - k) as f64 * bbox.min.z;
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::new(newx, y, newz);
                    min = Vec3::new(
                        min.x.min(tester.x),
                        min.y.min(tester.y),
                        min.z.min(tester.z),
                    );
                    max = Vec3::new(
                        max.x.max(tester.x),
                        max.y.max(tester.y),
                        max.z.max(tester.z),
                    );
                }
            }
        }


        RotateY {
            sin_theta,
            cos_theta,
            ptr: p,
            bbox: Some(Aabb::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin = Vec3::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
            r.origin.y,
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.z
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z
        );
        let rotated_r = Ray::new(origin, direction, r.time);
        self.ptr.hit(&rotated_r, t_min, t_max).map(|rec| {
            let p = Vec3::new(
                self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                rec.p.y,
                -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z
            );
            let normal = Vec3::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z
            );
            let mut rec2 = HitRecord {
                t: rec.t,
                u: rec.u,
                v: rec.v,
                mat_ptr: rec.mat_ptr,
                front_face: rec.front_face,
                normal,
                p,
            };
            rec2.set_face_normal(&rotated_r, &normal);
            rec2
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.bbox
    }
}
