use crate::math::Vec3;
use crate::physics::{HitRecord, Material, Ray};
use crate::textures::Texture;

pub struct DiffuseLight<T: Texture> {
    emit: T,
}

impl<T: Texture> DiffuseLight<T> {
    pub fn new(texture: T) -> Self {
        DiffuseLight { emit: texture }
    }
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(
        &self,
        _r: &Ray,
        _record: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.texture(u, v, p)
    }
}
