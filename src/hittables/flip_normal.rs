use crate::physics::{HitRecord, Hitable, Ray, AABB};

pub struct FlipNormal {
    hittable: Box<dyn Hitable>,
}

impl FlipNormal {
    pub fn new<S: Hitable + 'static>(hitable: S) -> Self {
        FlipNormal {
            hittable: Box::new(hitable),
        }
    }
}

impl Hitable for FlipNormal {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.hittable.hit(r, t_min, t_max) {
            Some(hit) => {
                let mut hit = hit;
                hit.normal = -hit.normal;
                Some(hit)
            }
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hittable.bounding_box(t0, t1)
    }
}
