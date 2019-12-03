use crate::math::Vec3;
use crate::textures::Texture;

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    nx: usize,
    ny: usize,
}

impl ImageTexture {
    pub fn new(data: Vec<u8>, nx: usize, ny: usize) -> Self {
        ImageTexture { data, nx, ny }
    }
}

impl Texture for ImageTexture {
    fn texture(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let i = (u * self.nx as f32).max(0.0).min(self.nx as f32 - 1.0);
        let j = (((1.0 - v) * self.ny as f32) - 0.001)
            .max(0.0)
            .min(self.nx as f32 - 1.0);
        let i = i as usize;
        let j = j as usize;

        let r = self.data[(3 * i + 3 * self.nx * j) as usize] as f32 / 255.0;
        let g = self.data[(3 * i + 3 * self.nx * j + 1) as usize] as f32 / 255.0;
        let b = self.data[(3 * i + 3 * self.nx * j + 2) as usize] as f32 / 255.0;
        Vec3::new(r, g, b)
    }
}
