use crate::math::Vec3;
use crate::physics::{HitRecord, Material, Ray};
use crate::textures::Texture;

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new<S: Texture + 'static>(texture: S) -> Self {
        DiffuseLight {
            emit: Box::new(texture),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        self.emit.texture(u, v, p)
    }
}
