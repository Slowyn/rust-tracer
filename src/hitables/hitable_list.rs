use crate::physics::{Hitable, HitRecord, Ray, AABB, surrounding_box};

pub struct HitableList {
    pub entities: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(entities: Vec<Box<dyn Hitable>>) -> Self {
        HitableList {
            entities,
        }
    }

    pub fn push<S: Hitable + 'static>(&mut self, entity: S) -> &mut Self {
        self.entities.push(Box::new(entity));
        self
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in self.entities.iter() {
            if let Some(rec) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let mut r_box: Option<AABB> = None;
        if self.entities.is_empty() {
            return None;
        }
        for entity in self.entities.iter() {
            match entity.bounding_box(t0, t1) {
                Some(aabb) => {
                    match r_box.as_mut() {
                        Some(v) => *v = surrounding_box(*v, aabb),
                        None => r_box = Some(aabb),
                    }
                },
                None => return None
            };
        }
        r_box
    }
}
