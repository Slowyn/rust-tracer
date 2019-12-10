use crate::hittables::{HitRecord, Hitable};
use crate::materials::Isotropic;
use crate::math::Vec3;
use crate::physics::{Material, Ray, AABB};
use crate::rand::prelude::*;
use crate::textures::Texture;

pub struct ConstantMedium<T, S>
where
    T: Hitable,
    S: Material,
{
    boundary: T,
    phase_function: S,
    density: f32,
}

impl<T, S> ConstantMedium<T, Isotropic<S>>
where
    T: Hitable,
    S: Texture,
{
    pub fn new(boundary: T, density: f32, texture: S) -> Self {
        let phase_function = Isotropic::new(texture);
        ConstantMedium {
            boundary,
            density,
            phase_function,
        }
    }
}

impl<T, S> Hitable for ConstantMedium<T, S>
where
    T: Hitable,
    S: Material,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.boundary
            .hit(r, std::f32::MIN, std::f32::MAX)
            .and_then(|mut rec1| {
                let mut rng = rand::thread_rng();
                let rec2 = self.boundary.hit(r, rec1.t + 0.0001, std::f32::MAX);
                let mut rec2 = rec2?;
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }
                let distance_inside_boundary = (rec2.t - rec1.t) * r.direction.length();
                let hit_distance = -(1.0 / self.density) * rng.gen::<f32>().ln();
                if hit_distance < distance_inside_boundary {
                    let t = rec1.t + hit_distance / r.direction.length();
                    return Some(HitRecord::new(
                        t,
                        r.point_at_parameter(t),
                        Vec3::new(1.0, 0.0, 0.0),
                        &self.phase_function,
                        0.0,
                        0.0,
                    ));
                }
                None
            })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
