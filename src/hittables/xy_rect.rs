use crate::hittables::{HitRecord, Hitable};
use crate::math::Vec3;
use crate::physics::{Material, Ray, AABB};

pub struct XYRect<T: Material> {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: T,
}

impl<T: Material> XYRect<T> {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: T) -> Self {
        XYRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl<T: Material> Hitable for XYRect<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z()) / r.direction.z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x() + t * r.direction.x();
        let y = r.origin.y() + t * r.direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord::new(
            t,
            r.point_at_parameter(t),
            Vec3::new(0.0, 0.0, 1.0),
            &self.material,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
