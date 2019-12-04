use crate::math::Vec3;
use crate::physics::{HitRecord, Ray};

pub trait Material: Sync {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
