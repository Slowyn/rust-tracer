use crate::math::Vec3;
use crate::physics::Ray;
use std::mem::swap;

#[inline]
fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

// Axis Aligned Bounding Box
#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> Option<(f32, f32)> {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let t1 = (self.max[a] - ray.origin[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return None
            }
        }
        Some((t_min, t_max))
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(
        ffmin(box0.min.x(), box1.min.x()),
        ffmin(box0.min.y(), box1.min.y()),
        ffmin(box0.min.z(), box1.min.z()),
    );
    let big = Vec3::new(
        ffmax(box0.max.x(), box1.max.x()),
        ffmax(box0.max.y(), box1.max.y()),
        ffmax(box0.max.z(), box1.max.z()),
    );
    AABB::new(small, big)
}
