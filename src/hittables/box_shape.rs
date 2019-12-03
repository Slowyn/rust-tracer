use crate::hittables::{FlipNormal, HittableList, XYRect, XZRect, YZRect};
use crate::math::Vec3;
use crate::physics::{HitRecord, Hitable, Material, Ray, AABB};

pub struct BoxShape<T: Material> {
    pmin: Vec3,
    pmax: Vec3,
    material: T,
    hit_objects: HittableList,
}


impl<T: Material + Clone + 'static> BoxShape<T> {
    pub fn new(p0: Vec3, p1: Vec3, material: T) -> Self {
        let mut objects = HittableList::new(Vec::with_capacity(6));
        objects.push(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        ));
        objects.push(FlipNormal::new(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.y(),
            material.clone(),
        )));
        objects.push(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material.clone(),
        ));
        objects.push(FlipNormal::new(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material.clone(),
        )));
        objects.push(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            material.clone(),
        ));

        objects.push(FlipNormal::new(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            material.clone(),
        )));

        BoxShape {
            pmin: p0,
            pmax: p1,
            material,
            hit_objects: objects,
        }
    }
}

impl<T: Material> Hitable for BoxShape<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit_objects.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(self.pmin, self.pmax))
    }
}
