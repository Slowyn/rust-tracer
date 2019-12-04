use crate::math::Vec3;

pub trait Texture: Sync {
    fn texture(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}
