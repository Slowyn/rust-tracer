use crate::math::Vec3;
use crate::physics::{HitRecord, Material, Ray};
use crate::random_in_unit_sphere;
use crate::textures::Texture;

pub struct Isotropic<T: Texture> {
    albedo: T,
}

impl<T: Texture> Isotropic<T> {
    pub fn new(albedo: T) -> Self {
        Isotropic { albedo }
    }
}

impl<T: Texture> Material for Isotropic<T> {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(record.p, random_in_unit_sphere(), r.time);
        *attenuation = self.albedo.texture(record.u, record.v, &record.p);
        true
    }
}
