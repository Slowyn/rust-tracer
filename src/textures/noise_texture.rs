use crate::math::{Perlin, Vec3};
use crate::textures::texture::Texture;

#[derive(Copy, Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn texture(&self, _u: f32, _v: f32, p: &Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f32::sin(self.scale * p.x() + 10.0 * self.noise.turb(&p, 7)))
    }
}
