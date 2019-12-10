use crate::math::Vec3;
use crate::physics::{Material, Ray, AABB};

mod box_shape;
mod bvh;
mod constant_medium;
mod flip_normal;
mod hittable_list;
mod moving_sphere;
mod rotate_y;
mod sphere;
mod translate;
mod xy_rect;
mod xz_rect;
mod yz_rect;

pub use self::{
    box_shape::*, bvh::*, constant_medium::*, flip_normal::*, hittable_list::*, moving_sphere::*,
    rotate_y::*, sphere::*, translate::*, xy_rect::*, xz_rect::*, yz_rect::*,
};

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub u: f32,
    pub v: f32,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a dyn Material, u: f32, v: f32) -> Self {
        HitRecord {
            t,
            p,
            normal,
            material,
            u,
            v,
        }
    }
}

pub trait Hitable: Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
