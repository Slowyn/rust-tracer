use crate::hitables::HitableList;
use crate::physics::{surrounding_box, HitRecord, Hitable, Ray, AABB};
use crate::rand::prelude::*;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

// Bounding Volume Hierarchy
pub struct BVHNode {
    pub b_box: AABB,
    pub left: Box<dyn Hitable>,
    pub right: Box<dyn Hitable>,
}

impl BVHNode {
    pub fn new(h_list: HitableList, t0: f32, t1: f32) -> BVHNode {
        let mut h_list: HitableList = h_list;
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0, 3);

        let compare_fn = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Incorrect axis choice"),
        };
        let left;
        let right;

        h_list.entities.sort_by(compare_fn);

        if h_list.entities.len() == 1 {
            left = h_list.entities[0].clone();
            right = h_list.entities[0].clone();
        } else if h_list.entities.len() == 2 {
            left = h_list.entities[0].clone();
            right = h_list.entities[1].clone();
        } else {
            let h_list2_entities = h_list.entities.split_off(h_list.entities.len() / 2);
            left = Box::new(BVHNode::new(h_list, t0, t1));
            right = Box::new(BVHNode::new(
                HitableList {
                    entities: h_list2_entities,
                },
                t0,
                t1,
            ));
        }

        let (box_left_o, box_right_o) = (left.bounding_box(t0, t1), right.bounding_box(t0, t1));
        if box_left_o.is_none() || box_right_o.is_none() {
            panic!("no bounding box in bvh_node constructor");
        }

        BVHNode {
            left,
            right,
            b_box: surrounding_box(box_left_o.unwrap(), box_right_o.unwrap()),
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.b_box.hit(r, t_min, t_max) {
            Some((min, max)) => {
                let hit_left = self.left.hit(r, min, max);
                let hit_right = self.right.hit(r, min, max);
                match (hit_left, hit_right) {
                    (Some(left_rec), Some(right_rec)) => Some(if left_rec.t < right_rec.t {
                        left_rec
                    } else {
                        right_rec
                    }),
                    (Some(left_rec), None) => Some(left_rec),
                    (None, Some(right_rec)) => Some(right_rec),
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.b_box)
    }
}

fn box_x_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    let (box_left, box_right) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0));
    if box_left.is_none() || box_right.is_none() {
        panic!("No bounding box");
    }
    let (box_left, box_right) = (box_left.unwrap(), box_right.unwrap());
    if box_left.min.x() - box_right.min.x() < 0.0 {
        Less
    } else {
        Greater
    }
}

fn box_y_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    let (box_left, box_right) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0));
    if box_left.is_none() || box_right.is_none() {
        panic!("No bounding box");
    }
    let (box_left, box_right) = (box_left.unwrap(), box_right.unwrap());
    if box_left.min.y() - box_right.min.y() < 0.0 {
        Less
    } else {
        Greater
    }
}

fn box_z_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
    let (box_left, box_right) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0));
    if box_left.is_none() || box_right.is_none() {
        panic!("No bounding box");
    }
    let (box_left, box_right) = (box_left.unwrap(), box_right.unwrap());
    if box_left.min.z() - box_right.min.z() < 0.0 {
        Less
    } else {
        Greater
    }
}
