use crate::physics::{HitRecord, Hitable, Ray, AABB};

pub struct FlipNormal<T: Hitable> {
    hittable: T,
}

impl<T: Hitable> FlipNormal<T> {
    pub fn new(hittable: T) -> Self {
        FlipNormal {
            hittable,
        }
    }
}

impl<T: Hitable> Hitable for FlipNormal<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hittable.hit(r, t_min, t_max).map(|mut hit| {
            hit.normal = -hit.normal;
            hit
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }
}
