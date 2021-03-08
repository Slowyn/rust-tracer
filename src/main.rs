mod hittables;
mod materials;
mod math;
mod physics;
mod scene;
mod textures;

extern crate image;
extern crate indicatif;
extern crate rand;
extern crate rayon;

use crate::hittables::{
    BoxShape, ConstantMedium, FlipNormal, RotateY, Translate, XYRect, XZRect, YZRect, BVH,
};
use crate::materials::DiffuseLight;
use crate::textures::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use hittables::{Hitable, HittableList, MovingSphere, Sphere};
use image::io::Reader as ImageReader;
use image::{ImageBuffer, RgbImage, ImageResult};
use indicatif::ProgressBar;
use materials::{Dielectric, Lambertian, Metal};
use math::Vec3;
use physics::{Camera, Ray};
use rand::prelude::*;
use rayon::prelude::*;
use scene::Scene;
use std::env;
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

#[allow(dead_code)]
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
    scene.push(boundary1);
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

    let image = ImageReader::open("./earthmap.jpg")
        .expect("Failed to open file")
        .decode()
        .expect("Failed to decode image")
        .into_rgb8();

    let earth = Sphere::new(
        100.0,
        Vec3::new(400.0, 200.0, 400.0),
        Lambertian::new(ImageTexture::new(
            image.as_raw().to_vec(),
            image.width() as usize,
            image.height() as usize,
        )),
    );
    scene.push(earth);

    let noise_sphere = Sphere::new(
        80.0,
        Vec3::new(220.0, 280.0, 300.0),
        Lambertian::new(NoiseTexture::new(0.1)),
    );
    scene.push(noise_sphere);

    let ns = 1000;
    for _ in 0..ns {
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

fn main() -> ImageResult<()> {
    let width = 1920;
    let height = 1080;
    let rays_per_pixel = match env::args().nth(1) {
        Some(x) => x
            .parse::<u32>()
            .expect("Expected to get a number of rays per pixel"),
        None => 100,
    };
    println!("Rays per pixel: {:?}", rays_per_pixel);
    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let aperture: f32 = 0.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        width as f32 / height as f32,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    let world = final_scene();
    let scene = Scene::new(camera, world, width, height, rays_per_pixel);
    let pixels = scene.render();

    let mut image_buffer: RgbImage = ImageBuffer::new(width, height);
    for (index, pixel) in image_buffer.pixels_mut().enumerate() {
        let pixel_color = pixels[index];
        *pixel = image::Rgb([pixel_color.0, pixel_color.1, pixel_color.2]);
    }
    image_buffer.save("img.png")
}
