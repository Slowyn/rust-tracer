use crate::physics::{Hitable, HitRecord, Ray};

impl Hitable for Vec<Box<dyn Hitable>> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable in self.iter() {
            if hitable.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *record = tmp_rec;
            }
        }
        hit_anything
    }
}