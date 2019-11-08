use crate::math::{dot, Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0)
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * *n
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
