use crate::math::Vec3;
use crate::physics::{HitRecord, Hitable, Material, Ray, AABB};

pub struct XZRect<T: Material> {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: T,
}

impl<T: Material> XZRect<T> {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: T) -> Self {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl<T: Material> Hitable for XZRect<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.y()) / r.direction.y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x() + t * r.direction.x();
        let z = r.origin.z() + t * r.direction.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            Vec3::new(0.0, 1.0, 0.0),
            &self.material,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}
