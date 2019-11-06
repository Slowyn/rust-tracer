use crate::physics::{Material, HitRecord, Ray, refract, reflect};
use crate::math::{Vec3, dot};

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric {
            ref_idx,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, record: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let reflected = reflect(&r.direction, &record.normal);
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        if dot(&r.direction, &record.normal) > 0.0 {
            outward_normal = -record.normal;
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
        }

        match refract(&r.direction, &outward_normal, ni_over_nt) {
            Some(refracted) => {
                *scattered = Ray::new(record.p, refracted);
                true
            },
            None => {
                *scattered = Ray::new(record.p, reflected);
                false
            },
        }
    }
}