use crate::physics::{Material, Ray, HitRecord};
use crate::math::Vec3;
use crate::random_in_unit_sphere;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Lambertian {
            albedo: Vec3::new(x, y, z),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}
