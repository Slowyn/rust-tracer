use crate::math::Vec3;
use crate::textures::Texture;

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new<T: Texture + 'static, S: Texture + 'static>(odd: T, even: S) -> Self {
        CheckerTexture {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }
}

impl Texture for CheckerTexture {
    fn texture(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.texture(u, v, p)
        } else {
            self.even.texture(u, v, p)
        }
    }
}
