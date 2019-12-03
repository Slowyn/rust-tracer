mod hittables;
mod materials;
mod math;
mod physics;
mod textures;

extern crate rand;
extern crate stb_image;

use crate::hittables::{FlipNormal, XYRect, XZRect, YZRect, BoxShape};
use crate::materials::DiffuseLight;
use crate::textures::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use hittables::{HittableList, MovingSphere, Sphere};
use materials::{Dielectric, Lambertian, Metal};
use math::Vec3;
use physics::{Camera, Hitable, Ray};
use rand::prelude::*;
use stb_image::image;
use stb_image::image::LoadResult;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
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

fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let n = 500;
    let mut scene = HittableList::new(Vec::with_capacity(n));

    let checker = CheckerTexture::new(
        ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
        ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
    );

    scene.push(Sphere::new(
        1000.0,
        Vec3::new(0.0, -1000.0, 0.0),
        Lambertian::new(checker),
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
                        Lambertian::new(ConstantTexture::new(Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))),
                        0.0,
                        1.0,
                    ));
                } else if choose_mat < 0.95 {
                    // Metal
                    scene.push(Sphere::new(
                        0.2,
                        center,
                        Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        ),
                    ));
                } else {
                    // Glass
                    scene.push(Sphere::new(0.2, center, Dielectric::new(1.5)));
                }
            }
        }
    }
    scene.push(Sphere::new(
        1.0,
        Vec3::new(0.0, 1.0, 0.0),
        Dielectric::new(1.5),
    ));
    scene.push(Sphere::new(
        1.0,
        Vec3::new(-4.0, 1.0, 0.0),
        Lambertian::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))),
    ));
    scene.push(Sphere::new(
        1.0,
        Vec3::new(4.0, 1.0, 0.0),
        Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0),
    ));
    scene
}

fn two_spheres() -> HittableList {
    let checker = CheckerTexture::new(
        ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
        ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
    );
    let mut scene = HittableList::new(Vec::with_capacity(2));
    scene.push(Sphere::new(
        10.0,
        Vec3::new(0.0, -10.0, 0.0),
        Lambertian::new(checker),
    ));
    let checker = CheckerTexture::new(
        ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1)),
        ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9)),
    );
    scene.push(Sphere::new(
        10.0,
        Vec3::new(0.0, 10.0, 0.0),
        Lambertian::new(checker),
    ));
    scene
}

fn earth() -> HittableList {
    let path = Path::new("./earthmap.jpg");
    let load_result = image::load(path);
    let image = match load_result {
        LoadResult::ImageU8(image) => image,
        _ => panic!("Image was not loaded!"),
    };
    let mut scene = HittableList::new(Vec::with_capacity(2));
    scene.push(Sphere::new(
        2.0,
        Vec3::new(0.0, 0.0, 0.0),
        Lambertian::new(ImageTexture::new(
            image.data,
            image.width,
            image.height,
        )),
    ));
    scene
}

fn two_perlin_spheres() -> HittableList {
    let pertext = NoiseTexture::new(3.1);
    let mut scene = HittableList::new(Vec::with_capacity(2));
    scene.push(Sphere::new(
        1000.0,
        Vec3::new(0.0, -1000.0, 0.0),
        Lambertian::new(pertext),
    ));
    let pertext = NoiseTexture::new(3.1);
    scene.push(Sphere::new(
        2.0,
        Vec3::new(0.0, 2.0, 0.0),
        Lambertian::new(pertext),
    ));
    scene
}

fn simple_light() -> HittableList {
    let pertext = NoiseTexture::new(3.1);
    let mut scene = HittableList::new(Vec::with_capacity(4));
    scene.push(Sphere::new(
        1000.0,
        Vec3::new(0.0, -1000.0, 0.0),
        Lambertian::new(pertext),
    ));
    let pertext = NoiseTexture::new(3.1);
    scene.push(Sphere::new(
        2.0,
        Vec3::new(0.0, 2.0, 0.0),
        Lambertian::new(pertext),
    ));

    scene.push(Sphere::new(
        2.0,
        Vec3::new(0.0, 2.0, 0.0),
        Lambertian::new(pertext),
    ));

    scene.push(Sphere::new(
        2.0,
        Vec3::new(0.0, 7.0, 0.0),
        DiffuseLight::new(ConstantTexture::new(Vec3::new(
            4.0, 4.0, 4.0,
        ))),
    ));

    scene.push(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        DiffuseLight::new(ConstantTexture::new(Vec3::new(
            4.0, 4.0, 4.0,
        ))),
    ));

    scene
}

fn cornell_box() -> HittableList {
    let mut scene = HittableList::new(Vec::with_capacity(7));
    let red = Lambertian::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)));

    scene.push(FlipNormal::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green,
    )));
    scene.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    scene.push(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light,
    ));
    scene.push(FlipNormal::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    scene.push(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    scene.push(FlipNormal::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    scene.push(BoxShape::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white.clone(),
    ));
    scene.push(BoxShape::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white.clone(),
    ));

    scene
}

fn main() -> std::io::Result<()> {
    let nx: i32 = 500;
    let ny: i32 = 250;
    let ns: i32 = 1000;
    let capacity = (nx * ny) as usize;
    let mut content = String::new();
    content.push_str(format!("P3\n{} {}\n255\n", nx, ny).as_str());

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let aperture: f32 = 0.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        nx as f32 / ny as f32,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    let world = cornell_box();

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

            let ir = (255.99 * col.r()) as u8;
            let ig = (255.99 * col.g()) as u8;
            let ib = (255.99 * col.b()) as u8;
            content.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }
    let mut image = File::create("img.ppm")?;
    image.write_all(content.as_bytes())
}
