use crate::physics::{Hitable, HitRecord, Ray};

pub struct HitableList {
    entities: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(entities: Vec<Box<dyn Hitable>>) -> Self {
        HitableList {
            entities,
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hitable in self.entities.iter() {
            match hitable.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    tmp_rec = Some(rec);
                },
                None => {},
            }
        }
        tmp_rec
    }
}