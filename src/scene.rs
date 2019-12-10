use crate::hittables::{Hitable, HittableList};
use crate::math::Vec3;
use crate::physics::{Camera, Ray};
use rand::prelude::*;
use rayon::prelude::*;

pub struct Scene {
    camera: Camera,
    objects: HittableList,
    width: u32,
    height: u32,
    rays_per_pixel: u32,
}

impl Scene {
    pub fn new(
        camera: Camera,
        objects: HittableList,
        width: u32,
        height: u32,
        rays_per_pixel: u32,
    ) -> Self {
        Scene {
            camera,
            objects,
            width,
            height,
            rays_per_pixel,
        }
    }

    pub fn render(&self) -> Vec<u8> {
        let nx = self.width;
        let ny = self.height;
        let ns = self.rays_per_pixel;
        (0..ny)
            .into_par_iter()
            .rev()
            .flat_map(|y| {
                (0..nx)
                    .flat_map(|x| {
                        let mut col = Vec3::default();
                        for _s in 0..ns {
                            let mut rng = rand::thread_rng();
                            let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                            let v = (y as f32 + rng.gen::<f32>()) / ny as f32;
                            let ray = self.camera.get_ray(u, v);
                            col += color(&ray, &self.objects, 0);
                        }
                        col /= ns as f32;
                        col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

                        let r = (255.99 * col.r()).min(255.0) as u8;
                        let g = (255.99 * col.g()).min(255.0) as u8;
                        let b = (255.99 * col.b()).min(255.0) as u8;
                        vec![r, g, b]
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>()
    }
}

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    match world.hit(&r, 0.001, std::f32::MAX) {
        Some(rec) => {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);
            if depth < 50
                && rec
                    .material
                    .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                emitted + attenuation * color(&scattered, world, depth + 1)
            } else {
                emitted
            }
        }
        None => Vec3::default(),
    }
}
