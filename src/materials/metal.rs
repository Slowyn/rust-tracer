use crate::physics::{Material, Ray, HitRecord, reflect};
use crate::math::{Vec3, dot};
use crate::random_in_unit_sphere;

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {

    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction.unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(), r_in.time);
        *attenuation = self.albedo;
        dot(&scattered.direction, &rec.normal) > 0.0
    }
}
