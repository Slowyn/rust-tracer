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

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<(f32, f32)> {
        let min = self.min;
        let max = self.max;

        let mut r_tmin = tmin;
        let mut r_tmax = tmin;

        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (min[a] - r.origin[a]) * inv_d;
            let mut t1 = (max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            r_tmin = ffmax(t0, tmin);
            r_tmax = ffmin(t1, tmax);
            if r_tmax <= r_tmin {
                return None;
            }
        }
        Some((r_tmin, r_tmax))
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Vec3::new(
        ffmin(box0.min.x(), box1.min.x()),
        ffmin(box0.min.y(), box1.min.y()),
        ffmin(box0.min.z(), box1.min.z()),
    );
    let big = Vec3::new(
        ffmax(box0.min.x(), box1.min.x()),
        ffmax(box0.min.y(), box1.min.y()),
        ffmax(box0.min.z(), box1.min.z()),
    );
    AABB::new(small, big)
}
