use std::cmp::Ordering;
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::aabb::{Aabb, surrounding_box};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;

#[derive(Clone)]
pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    r#box: Aabb
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.r#box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right =
            match hit_left {
                Some(ref hl) => self.right.hit(r, t_min, hl.t),
                None =>  self.right.hit(r, t_min,t_max),
            };
        if hit_right.is_some() {
            hit_right
        } else if hit_left.is_some() {
            hit_left
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<Aabb> {
        Some(self.r#box)
    }
}

impl BvhNode {
    pub fn new(
        rng: &mut ThreadRng,
        mut hittable_list: HittableList,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64
    ) -> BvhNode {
        let objects = &mut hittable_list.objects;
        let axis = rng.gen_range(0..=2);
        let comparator =
            if axis == 0 {
                box_x_compare
            } else if axis == 1 {
                box_y_compare
            } else {
                box_z_compare
            };

        let object_span = end - start;

        let (left, right) =
            if object_span == 1 {
                let x = &objects[start];
                (x.clone_box(), x.clone_box())
            } else if object_span == 2 {
                let start_object= &objects[start];
                let next_object= &objects[start+1];

                if comparator(start_object, next_object).is_gt() {
                    (start_object.clone_box(), next_object.clone_box())
                } else {
                    (next_object.clone_box(), start_object.clone_box())
                }
            } else {
                let (_, temp) = &mut objects.split_at_mut(start);
                let (temp2, _) = temp.split_at_mut(end - start);
                temp2.sort_by(comparator);

                let mid = start + object_span / 2;
                (
                    Box::new(BvhNode::new(rng, HittableList { objects: objects.to_vec() }, start, mid, time0, time1)) as Box<dyn Hittable>,
                    Box::new(BvhNode::new(rng, HittableList { objects: objects.to_vec() }, mid, end, time0, time1)) as Box<dyn Hittable>
                )
            };
        let box_left = left.bounding_box(time0, time1).unwrap();
        let box_right =right.bounding_box(time0, time1).unwrap();
        BvhNode {
            left: left,
            right: right,
            r#box: surrounding_box(box_left, box_right)
        }
    }
}
#[inline]
fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: i32) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    box_a.min.d(axis).partial_cmp(&box_b.min.d(axis)).unwrap()
}

fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
