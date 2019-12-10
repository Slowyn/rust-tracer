use crate::hittables::{FlipNormal, HittableList, XYRect, XZRect, YZRect};
use crate::hittables::{HitRecord, Hitable};
use crate::math::Vec3;
use crate::physics::{Material, Ray, AABB};

pub struct BoxShape {
    pmin: Vec3,
    pmax: Vec3,
    hit_objects: HittableList,
}

impl BoxShape {
    pub fn new<T: Material + Clone + 'static>(p_min: Vec3, p_max: Vec3, material: T) -> Self {
        let mut objects = HittableList::new(Vec::with_capacity(6));
        objects.push(XYRect::new(
            p_min.x(),
            p_max.x(),
            p_min.y(),
            p_max.y(),
            p_max.z(),
            material.clone(),
        ));
        objects.push(FlipNormal::new(XYRect::new(
            p_min.x(),
            p_max.x(),
            p_min.y(),
            p_max.y(),
            p_min.z(),
            material.clone(),
        )));
        objects.push(XZRect::new(
            p_min.x(),
            p_max.x(),
            p_min.z(),
            p_max.z(),
            p_max.y(),
            material.clone(),
        ));
        objects.push(FlipNormal::new(XZRect::new(
            p_min.x(),
            p_max.x(),
            p_min.z(),
            p_max.z(),
            p_min.y(),
            material.clone(),
        )));
        objects.push(YZRect::new(
            p_min.y(),
            p_max.y(),
            p_min.z(),
            p_max.z(),
            p_max.x(),
            material.clone(),
        ));

        objects.push(FlipNormal::new(YZRect::new(
            p_min.y(),
            p_max.y(),
            p_min.z(),
            p_max.z(),
            p_min.x(),
            material.clone(),
        )));

        BoxShape {
            pmin: p_min,
            pmax: p_max,
            hit_objects: objects,
        }
    }
}

impl Hitable for BoxShape {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hit_objects.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(self.pmin, self.pmax))
    }
}
