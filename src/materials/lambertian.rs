use crate::math::Vec3;
use crate::physics::{HitRecord, Material, Ray};
use crate::random_in_unit_sphere;
use crate::textures::Texture;

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new<S: Texture + 'static>(texture: S) -> Self {
        Lambertian {
            albedo: Box::new(texture),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p, r_in.time);
        let (u, v) = rec.uv;
        *attenuation = self.albedo.texture(u, v, &rec.p);
        true
    }
}
