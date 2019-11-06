use crate::math::{Vec3, dot};
use crate::physics::{Ray};
use crate::rand::prelude::*;

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    while dot(&p, &p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}


pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);
        Camera {
            lower_left_corner: lookfrom - focus_dist * half_width * u - focus_dist * half_height * v - focus_dist * w,
            horizontal: 2.0 * focus_dist * half_width * u,
            vertical: 2.0 * focus_dist * half_height * v,
            origin: lookfrom,
            lens_radius,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            lens_radius: 2.0,
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
