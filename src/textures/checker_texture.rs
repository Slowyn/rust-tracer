use crate::math::Vec3;
use crate::textures::Texture;

#[derive(Clone)]
pub struct CheckerTexture<T: Texture + Clone, S: Texture + Clone> {
    odd: T,
    even: S,
}

impl<T: Texture + Clone, S: Texture + Clone> CheckerTexture<T, S> {
    pub fn new(odd: T, even: S) -> Self {
        CheckerTexture { odd, even }
    }
}

impl<T: Texture + Clone, S: Texture + Clone> Texture for CheckerTexture<T, S> {
    fn texture(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.texture(u, v, p)
        } else {
            self.even.texture(u, v, p)
        }
    }
}
