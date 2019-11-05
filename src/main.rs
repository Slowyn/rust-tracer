mod math;
mod physics;
mod hitables;

extern crate rand;

use std::fs::File;
use std::io::Write;
use math::{Vec3};
use physics::{Ray, Hitable, HitRecord, Camera};
use hitables::{Sphere};
use rand::prelude::*;


fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color(r: &Ray, world: &Vec<Box<dyn Hitable>>) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.001, std::f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(&Ray::new(rec.p, target - rec.p), &world);
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}


fn main() -> std::io::Result<()> {
    let mut image = File::create("img.ppm")?;
    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100;
    let capacity = (nx * ny) as usize;
    let mut content = String::with_capacity(capacity);
    content.push_str(format!("P3\n{} {}\n255\n", nx, ny).as_str());

    let camera = Camera::default();

    let world: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(0.5, Vec3::new(0.0, 0.0, -1.0))),
        Box::new(Sphere::new(100.0, Vec3::new(0.0, -100.5, -1.0))),
    ];

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
                col += color(&ray, &world);
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
