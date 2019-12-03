use crate::math::{dot, Vec3};
use crate::physics::{HitRecord, Hitable, Material, Ray, AABB};
use std::f32::consts::PI;

pub struct Sphere<T: Material> {
    pub r: f32,
    pub center: Vec3,
    pub material: T,
}

impl<T: Material> Sphere<T> {
    pub fn new(radius: f32, center: Vec3, material: T) -> Self {
        Sphere {
            r: radius,
            center,
            material,
        }
    }
}

impl<T: Material> Hitable for Sphere<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let radius = self.r;
        let center = self.center;
        let oc = r.origin - center;
        let a = dot(&r.direction, &r.direction);
        let b = dot(&oc, &r.direction);
        let c = dot(&oc, &oc) - radius * radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - center) / radius;
                let (u, v) = self.get_uv(&p);
                return Some(HitRecord::new(temp, p, normal, &self.material, u, v));
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - center) / radius;
                let (u, v) = self.get_uv(&p);
                return Some(HitRecord::new(temp, p, normal, &self.material, u, v));
            }
        }
        None
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.r, self.r, self.r),
            self.center + Vec3::new(self.r, self.r, self.r),
        ))
    }

    fn get_uv(&self, p: &Vec3) -> (f32, f32) {
        let p = (*p - self.center) / self.r;
        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / PI;
        (u, v)
    }
}
