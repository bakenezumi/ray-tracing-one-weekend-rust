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
            let t0 = f64::min((self.min.dimension(a) - r.origin.dimension(a)) / r.direction.dimension(a),
                      (self.max.dimension(a) - r.origin.dimension(a)) / r.direction.dimension(a));
            let t1 = f64::max((self.min.dimension(a) - r.origin.dimension(a)) / r.direction.dimension(a),
                      (self.max.dimension(a) - r.origin.dimension(a)) / r.direction.dimension(a));
            tmin = f64::max(t0, tmin);
            tmax = f64::min(t1, tmax);
            if tmax <= tmin {
                return false
            }
        }
        true
    }
}
