use crate::math::{dot, Vec3};
use crate::physics::{surrounding_box, HitRecord, Hitable, Material, Ray, AABB};

pub struct MovingSphere {
    pub r: f32,
    pub center0: Vec3,
    pub center1: Vec3,
    pub material: Box<dyn Material>,
    pub time0: f32,
    pub time1: f32,
}

impl MovingSphere {
    pub fn new(
        radius: f32,
        center0: Vec3,
        center1: Vec3,
        material: Box<dyn Material>,
        time0: f32,
        time1: f32,
    ) -> Self {
        MovingSphere {
            r: radius,
            center0,
            center1,
            material,
            time0,
            time1,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let radius = self.r;
        let center = self.center(r.time);
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
                return Some(HitRecord::new(temp, p, normal, &*self.material, u, v));
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                let normal = (p - center) / radius;
                let (u, v) = self.get_uv(&p);
                return Some(HitRecord::new(temp, p, normal, &*self.material, u, v));
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(t0) - Vec3::new(self.r, self.r, self.r),
            self.center(t0) + Vec3::new(self.r, self.r, self.r),
        );
        let box1 = AABB::new(
            self.center(t1) - Vec3::new(self.r, self.r, self.r),
            self.center(t1) + Vec3::new(self.r, self.r, self.r),
        );
        Some(surrounding_box(box0, box1))
    }

    fn get_uv(&self, p: &Vec3) -> (f32, f32) {
        (0.0, 0.0)
    }
}
