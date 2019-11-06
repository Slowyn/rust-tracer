use crate::math::{Vec3};
use crate::physics::{Ray, HitRecord};

pub trait Material {
    fn scatter(&self, r: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}
