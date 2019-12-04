use crate::math::Vec3;
use crate::physics::{HitRecord, Hitable, Ray, AABB};
use std::f32::consts::PI;

pub struct RotateY<T: Hitable> {
    object: T,
    sin_theta: f32,
    cos_theta: f32,
    aabb: Option<AABB>,
}

impl<T: Hitable> RotateY<T> {
    pub fn new(object: T, angle: f32) -> Self {
        let radians = (PI / 100.0) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let aabb = object.bounding_box(0.0, 1.0);
        let mut min = Vec3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
        let mut max = Vec3::new(std::f32::MIN, std::f32::MIN, std::f32::MIN);
        let aabb = match aabb {
            Some(bbox) => {
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = i as f32 * bbox.max.x() + (1 - i) as f32 * bbox.min.x();
                            let y = j as f32 * bbox.max.y() + (1 - j) as f32 * bbox.min.y();
                            let z = k as f32 * bbox.max.z() + (1 - k) as f32 * bbox.min.z();
                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;
                            let tester = Vec3::new(new_x, y, new_z);
                            for c in 0..3 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }
                Some(AABB::new(min, max))
            }
            None => None,
        };
        RotateY {
            object,
            sin_theta,
            cos_theta,
            aabb,
        }
    }
}

impl<T: Hitable> Hitable for RotateY<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = Vec3::new(
            self.cos_theta * r.origin.x() - self.sin_theta * r.origin.z(),
            r.origin.y(),
            self.sin_theta * r.origin.x() + self.cos_theta * r.origin.z(),
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction.x() - self.sin_theta * r.direction.z(),
            r.direction.y(),
            self.sin_theta * r.direction.x() + self.cos_theta * r.direction.z(),
        );

        let rotated_ray = Ray::new(origin, direction, r.time);
        self.object
            .hit(&rotated_ray, t_min, t_max)
            .map(|mut hit_record| {
                let p = hit_record.p;
                let p = Vec3::new(
                    self.cos_theta * p.x() + self.sin_theta * p.z(),
                    p.y(),
                    -self.sin_theta * p.x() + self.cos_theta * p.z(),
                );
                let normal = hit_record.normal;
                let normal = Vec3::new(
                    self.cos_theta * normal.x() + self.sin_theta * normal.z(),
                    normal.y(),
                    -self.sin_theta * normal.x() + self.cos_theta * normal.z(),
                );
                hit_record.p = p;
                hit_record.normal = normal;
                hit_record
            })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.aabb
    }
}
