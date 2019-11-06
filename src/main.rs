mod math;
mod physics;
mod hitables;
mod materials;

extern crate rand;

use std::fs::File;
use std::io::Write;
use math::{Vec3};
use physics::{Ray, Hitable, Camera};
use hitables::{Sphere, HitableList};
use materials::{Metal, Lambertian};
use rand::prelude::*;
use crate::materials::Dielectric;


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
            let mut scattered = Ray::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            );
            let mut attenuation = Vec3::default();
            if depth < 50 && rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
                attenuation * color(&scattered, &world, depth + 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        },
        None => {
            let unit_direction = r.direction.unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}


fn main() -> std::io::Result<()> {
    let mut image = File::create("img.ppm")?;
    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100;
    let capacity = (nx * ny) as usize;
    let mut content = String::with_capacity(capacity);
    content.push_str(format!("P3\n{} {}\n255\n", nx, ny).as_str());


    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let focus_dist = (lookfrom - lookat).length();
    let aperture: f32 = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32 / ny as f32,
        aperture,
        focus_dist,
    );

    let world: HitableList = HitableList::new(vec![
        Box::new(
            Sphere::new(
                0.5,
                Vec3::new(0.0, 0.0, -1.0),
                Box::new(Lambertian::new(0.1, 0.2, 0.5))
            ),
        ),
        Box::new(
            Sphere::new(
                100.0,
                Vec3::new(0.0, -100.5, -1.0),
                Box::new(Lambertian::new(0.8, 0.8, 0.0))
            ),
        ),
        Box::new(
            Sphere::new(
                0.5,
                Vec3::new(1.0, 0.0, -1.0),
                Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))
            ),
        ),
        Box::new(
            Sphere::new(
                0.5,
                Vec3::new(-1.0, 0.0, -1.0),
                Box::new(Dielectric::new(1.5)),
            )
        ),
        Box::new(
            Sphere::new(
                -0.45,
                Vec3::new(-1.0, 0.0, -1.0),
                Box::new(Dielectric::new(1.5)),
            )
        ),
    ]);

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
