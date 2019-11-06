use crate::physics::{Hitable, HitRecord, Ray, Material};
use crate::math::{Vec3, dot};

pub struct Sphere {
    pub r: f32,
    pub center: Vec3,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(radius: f32, center: Vec3, material: Box<dyn Material>) -> Self {
        Sphere {
            r: radius,
            center,
            material,
        }
    }
}

impl Hitable for Sphere {
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
                return Some(
                    HitRecord::new(
                        temp,
                        p,
                        normal,
                        &*self.material,
                    )
                );
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - center) / radius;
                return Some(
                    HitRecord::new(
                        temp,
                        p,
                        normal,
                        &*self.material,
                    )
                );
            }
        }
        return None;
    }
}