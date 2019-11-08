use crate::math::Vec3;
use crate::textures::texture::Texture;

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn texture(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color
    }
}
