use crate::math::Vec3;
use crate::textures::texture::Texture;

pub struct ConstantTexture {
    color: Vec3,
}

impl ConstantTexture {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        ConstantTexture {
            color: Vec3(r, g, b),
        }
    }
}

impl Texture for ConstantTexture {
    fn texture(&self, u: f32, v: f32, r: &Vec3) -> Vec3 {
        self.color
    }
}
