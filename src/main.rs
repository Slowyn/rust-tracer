mod hitables;
mod materials;
mod math;
mod physics;
mod textures;

extern crate rand;

use crate::textures::{CheckerTexture, ConstantTexture};
use hitables::{HitableList, MovingSphere, Sphere};
use materials::{Dielectric, Lambertian, Metal};
use math::Vec3;
use physics::{Camera, Hitable, Ray};
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color(r: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    match world.hit(&r, 0.001, std::f32::MAX) {
        Some(rec) => {
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if depth < 50
                && rec
                    .material
                    .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                attenuation * color(&scattered, &world, depth + 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn get_random_scene() -> HitableList {
    let mut rng = rand::thread_rng();
    let n = 500;
    let mut scene = HitableList::new(Vec::with_capacity(n));

    let checker = CheckerTexture::new(
        ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
        ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
    );

    scene.push(Sphere::new(
        1000.0,
        Vec3::new(0.0, -1000.0, 0.0),
        Box::new(Lambertian::new(checker)),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let af = a as f32;
            let bf = b as f32;
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                af + 0.9 + rng.gen::<f32>(),
                0.2,
                bf + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    scene.push(MovingSphere::new(
                        0.2,
                        center,
                        center + Vec3::new(0.0, 0.5 * rng.gen::<f32>(), 0.0),
                        Box::new(Lambertian::new(ConstantTexture::new(Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )))),
                        0.0,
                        1.0,
                    ));
                } else if choose_mat < 0.95 {
                    // Metal
                    scene.push(Sphere::new(
                        0.2,
                        center,
                        Box::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    ));
                } else {
                    // Glass
                    scene.push(Sphere::new(0.2, center, Box::new(Dielectric::new(1.5))));
                }
            }
        }
    }
    scene.push(Sphere::new(
        1.0,
        Vec3::new(0.0, 1.0, 0.0),
        Box::new(Dielectric::new(1.5)),
    ));
    scene.push(Sphere::new(
        1.0,
        Vec3::new(-4.0, 1.0, 0.0),
        Box::new(Lambertian::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        )))),
    ));
    scene.push(Sphere::new(
        1.0,
        Vec3::new(4.0, 1.0, 0.0),
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ));
    scene
}

fn main() -> std::io::Result<()> {
    let mut image = File::create("img.ppm")?;
    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 5;
    let capacity = (nx * ny) as usize;
    let mut content = String::with_capacity(capacity);
    content.push_str(format!("P3\n{} {}\n255\n", nx, ny).as_str());

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture: f32 = 0.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    let world = get_random_scene();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _s in 0..ns {
                let mut rng = rand::thread_rng();
                let r1: f32 = rng.gen();
                let u = (i as f32 + r1) / nx as f32;
                let r2: f32 = rng.gen();
                let v = (j as f32 + r2) / ny as f32;
                let ray = camera.get_ray(u, v);
                let _p = ray.point_at_parameter(2.0);
                col += color(&ray, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

            let ir = (255.99 * col.r()) as i16;
            let ig = (255.99 * col.g()) as i16;
            let ib = (255.99 * col.b()) as i16;
            content.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }
    image.write_all(content.as_bytes())
}
