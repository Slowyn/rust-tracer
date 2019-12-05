mod hittables;
mod materials;
mod math;
mod physics;
mod textures;

extern crate indicatif;
extern crate rand;
extern crate rayon;
extern crate stb_image;

use crate::hittables::{
    BoxShape, ConstantMedium, FlipNormal, RotateY, Translate, XYRect, XZRect, YZRect, BVH,
};
use crate::materials::DiffuseLight;
use crate::textures::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use hittables::{HittableList, MovingSphere, Sphere};
use indicatif::ProgressBar;
use materials::{Dielectric, Lambertian, Metal};
use math::Vec3;
use physics::{Camera, Hitable, Ray};
use rand::prelude::*;
use rayon::prelude::*;
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
        Lambertian::new(ConstantTexture::new(Vec3::new(0.4, 0.2, 0.1))),
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
        Lambertian::new(ImageTexture::new(image.data, image.width, image.height)),
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
        DiffuseLight::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0))),
    ));

    scene.push(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        DiffuseLight::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0))),
    ));

    scene
}

fn cornell_box() -> HittableList {
    let mut scene = HittableList::new(Vec::with_capacity(8));
    let red = Lambertian::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(ConstantTexture::new(Vec3::new(7.0, 7.0, 7.0)));

    scene.push(FlipNormal::new(YZRect::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )));
    scene.push(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    scene.push(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light));
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
    let box1 = Translate::new(
        RotateY::new(
            BoxShape::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let box2 = Translate::new(
        RotateY::new(
            BoxShape::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );

    scene.push(ConstantMedium::new(
        box1,
        0.01,
        ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0)),
    ));
    scene.push(ConstantMedium::new(
        box2,
        0.01,
        ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0)),
    ));

    scene
}

fn final_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut scene = HittableList::new(Vec::with_capacity(30));
    let mut box_list1 = HittableList::new(Vec::with_capacity(10000));
    let mut box_list2 = HittableList::new(Vec::with_capacity(10000));
    let white = Lambertian::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)));
    let ground = Lambertian::new(ConstantTexture::new(Vec3::new(0.48, 0.83, 0.53)));
    let nb = 20;
    for i in 0..nb {
        for j in 0..nb {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rng.gen::<f32>() + 0.01);
            let z1 = z0 + w;
            box_list1.push(BoxShape::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    let box_list1_bvh = BVH::new(box_list1.entities, 0.0, 1.0);
    scene.push(box_list1_bvh);

    let light = DiffuseLight::new(ConstantTexture::new(Vec3::new(7.0, 7.0, 7.0)));
    let light_source = XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light);
    scene.push(light_source);

    let center = Vec3::new(400.0, 400.0, 200.0);
    let moving_sphere1 = MovingSphere::new(
        50.0,
        center,
        center + Vec3::new(30.0, 0.0, 0.0),
        Lambertian::new(ConstantTexture::new(Vec3::new(0.7, 0.3, 0.1))),
        0.0,
        1.0,
    );
    scene.push(moving_sphere1);
    let sphere1 = Sphere::new(50.0, Vec3::new(260.0, 150.0, 45.0), Dielectric::new(1.5));
    scene.push(sphere1);
    let sphere2 = Sphere::new(
        50.0,
        Vec3::new(0.0, 150.0, 45.0),
        Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0),
    );
    scene.push(sphere2);

    let boundary1 = Sphere::new(70.0, Vec3::new(360.0, 150.0, 145.0), Dielectric::new(1.5));
    scene.push(boundary1.clone());
    let constant_medium1 = ConstantMedium::new(
        boundary1,
        0.2,
        ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9)),
    );
    scene.push(constant_medium1);
    let boundary2 = Sphere::new(5000.0, Vec3::new(0.0, 0.0, 0.0), Dielectric::new(1.5));
    let constant_medium2 = ConstantMedium::new(
        boundary2,
        0.0001,
        ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0)),
    );
    scene.push(constant_medium2);

    let path = Path::new("./earthmap.jpg");
    let load_result = image::load(path);
    let image = match load_result {
        LoadResult::ImageU8(image) => image,
        _ => panic!("Image was not loaded!"),
    };

    let earth = Sphere::new(
        100.0,
        Vec3::new(400.0, 200.0, 400.0),
        Lambertian::new(ImageTexture::new(image.data, image.width, image.height)),
    );
    scene.push(earth);

    let noise_sphere = Sphere::new(
        80.0,
        Vec3::new(220.0, 280.0, 300.0),
        Lambertian::new(NoiseTexture::new(0.1)),
    );
    scene.push(noise_sphere);

    let ns = 1000;
    for i in 0..ns {
        box_list2.push(Sphere::new(
            10.0,
            Vec3::new(
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
                165.0 * rng.gen::<f32>(),
            ),
            white.clone(),
        ));
    }

    let transformed = Translate::new(
        RotateY::new(BVH::new(box_list2.entities, 0.0, 1.0), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    );

    scene.push(transformed);

    scene
}

fn main() -> std::io::Result<()> {
    let nx: i32 = 1366;
    let ny: i32 = 768;
    let ns: i32 = 5000;

    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
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

    let world = final_scene();

    let pixels = (0..ny)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..nx)
                .flat_map(|x| {
                    let mut col = Vec3::default();
                    for _s in 0..ns {
                        let mut rng = rand::thread_rng();
                        let r1: f32 = rng.gen();
                        let u = (x as f32 + r1) / nx as f32;
                        let r2: f32 = rng.gen();
                        let v = (y as f32 + r2) / ny as f32;
                        let ray = camera.get_ray(u, v);
                        col += color(&ray, &world, 0);
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
        .collect::<Vec<u8>>();

    let mut image = File::create("img.ppm")?;
    let capacity = (nx * ny) as usize;
    let mut content = String::new();
    content.push_str(format!("P3\n{} {}\n255\n", nx, ny).as_str());
    for pixel in pixels.chunks(3) {
        content.push_str(format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]).as_str())
    }
    image.write_all(content.as_bytes())
}
