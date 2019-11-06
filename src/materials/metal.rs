use crate::physics::{Material, Ray, HitRecord, reflect};
use crate::math::{Vec3, dot};

pub struct Metal {
    albedo: Vec3,
}

impl Metal {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Metal {
            albedo: Vec3::new(x, y, z),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction.unit_vector(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        dot(&scattered.direction, &rec.normal) > 0.0
    }
}
