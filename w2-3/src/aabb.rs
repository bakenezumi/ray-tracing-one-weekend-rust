use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Aabb {
    min: Point3,
    max: Point3
}

impl Aabb {
    pub fn new(min: Point3, max: Point3) -> Aabb {
        Aabb {
            min,
            max,
        }
    }

    pub fn hit(&self, r: Ray, min: f64, max: f64) -> bool {
        let mut tmin= max;
        let mut tmax = min;
        for a in 0..=2 {
            let inv_d = 1.0 / r.direction.d(a);
            let mut t0 = (self.min.d(a) - r.origin.d(a)) * inv_d;
            let mut t1 = (self.max.d(a) - r.origin.d(a)) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false
            }
        }
        true
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Point3::new(
        f64::min(box0.min.x, box1.min.x),
        f64::min(box0.min.y, box1.min.y),
        f64::min(box0.min.z, box1.min.z)
    );
    let big = Point3::new(
        f64::max(box0.max.x, box1.max.x),
        f64::max(box0.max.y, box1.max.y),
        f64::max(box0.max.z, box1.max.z)
    );
    Aabb::new(small, big)
}
