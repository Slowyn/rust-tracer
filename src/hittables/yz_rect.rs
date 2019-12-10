use crate::hittables::{HitRecord, Hitable};
use crate::math::Vec3;
use crate::physics::{Material, Ray, AABB};

pub struct YZRect<T: Material> {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: T,
}

impl<T: Material> YZRect<T> {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: T) -> Self {
        YZRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl<T: Material> Hitable for YZRect<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.x()) / r.direction.x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin.y() + t * r.direction.y();
        let z = r.origin.z() + t * r.direction.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            Vec3::new(1.0, 0.0, 0.0),
            &self.material,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
