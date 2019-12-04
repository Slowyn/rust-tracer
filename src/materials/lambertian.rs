use crate::math::Vec3;
use crate::physics::{HitRecord, Material, Ray};
use crate::random_in_unit_sphere;
use crate::textures::Texture;

#[derive(Clone)]
pub struct Lambertian<T: Texture + Clone> {
    albedo: T,
}

impl<T: Texture + Clone> Lambertian<T> {
    pub fn new(texture: T) -> Self {
        Lambertian { albedo: texture }
    }
}

impl<T: Texture + Clone> Material for Lambertian<T> {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p, r_in.time);
        *attenuation = self.albedo.texture(rec.u, rec.v, &rec.p);
        true
    }
}
