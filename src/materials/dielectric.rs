use crate::math::{dot, Vec3};
use crate::physics::{reflect, refract, schlick, HitRecord, Material, Ray};
use crate::rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut rng = rand::thread_rng();
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let reflected = reflect(&r.direction, &record.normal);
        let cosine: f32;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        if dot(&r.direction, &record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(&r.direction, &record.normal) / r.direction.length();
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -dot(&r.direction, &record.normal) / r.direction.length()
        }

        let refracted = refract(&r.direction, &outward_normal, ni_over_nt);

        let reflect_prob = match refracted {
            Some(_refracted) => schlick(cosine, self.ref_idx),
            None => 1.0,
        };

        let chance: f32 = rng.gen();
        if chance < reflect_prob {
            *scattered = Ray::new(record.p, reflected, r.time);
        } else {
            *scattered = Ray::new(record.p, refracted.unwrap(), r.time);
        }
        true
    }
}
