use crate::math::Vec3;
use crate::physics::ray::Ray;
use crate::physics::{Material, AABB};

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

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
    fn get_uv(&self, p: &Vec3) -> (f32, f32) {
        (0.0, 0.0)
    }
}
