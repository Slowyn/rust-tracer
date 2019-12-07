use crate::math::Vec3;
use crate::physics::{HitRecord, Hitable, Ray, AABB};

pub struct Translate<T: Hitable> {
    object: T,
    offset: Vec3,
}

impl<T: Hitable> Translate<T> {
    pub fn new(object: T, offset: Vec3) -> Self {
        Translate { object, offset }
    }
}

impl<T: Hitable> Hitable for Translate<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(r.origin - self.offset, r.direction, r.time);
        self.object
            .hit(&moved_ray, t_min, t_max)
            .map(|mut hit_record| {
                hit_record.p += self.offset;
                hit_record
            })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let aabb = self.object.bounding_box(t0, t1);
        aabb.map(|b| AABB::new(b.min + self.offset, b.max + self.offset))
    }
}
