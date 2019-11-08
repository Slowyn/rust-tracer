use crate::math::Vec3;

pub trait Texture {
    fn texture(&self, u: f32, v: f32, r: &Vec3) -> Vec3;
}
