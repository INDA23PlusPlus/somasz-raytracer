mod aabb;
mod camera;
mod hit;
mod material;
mod planes;
mod ray;
mod sphere;
mod vec;

use camera::Camera;
use hit::{Hittable, World};
use material::{Lambertian, Metal};
use planes::Plane;
use rand::*;
use ray::Ray;
use sphere::Sphere;
use std::io::{stderr, Write};
use std::sync::Arc;

use vec::{Color, Point3, Vec3};

use crate::material::DiffuseLight;

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;
    const SAMPLEPER_PIXEL: u64 = 1000;
    const MAX_DEPTH: u64 = 50;

    //World
    let mut world = World::new();
    let mat_ground_plane = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));
    let mat_light = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));

    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.5, 0.0, -1.5), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.5, 0.0, -1.5), 0.5, mat_right);
    let ground_plane = Plane::new(
        Vec3::new(0.0, 1.0, 0.0).normalized(),
        -0.5,
        mat_ground_plane,
    );
    let sphere_light = Sphere::new(Point3::new(0.0, 1.3, -1.3), 0.5, mat_light);

    world.push(Box::new(ground_plane));
    world.push(Box::new(sphere_light));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    //Camera
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 1.3),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO,
        Color::new(0.0, 0.0, 0.0), // Color::new(0.70, 0.80, 1.00),
    );

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");
    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {:3}", (IMAGE_HEIGHT - j - 1));
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLEPER_PIXEL {
                let radnom_v: f64 = rng.gen();
                let radnom_u: f64 = rng.gen();

                let u = ((i as f64) + radnom_u) / (IMAGE_WIDTH - 1) as f64;
                let v = ((j as f64) + radnom_v) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);

                pixel_color += camera.ray_color(&ray, &world, MAX_DEPTH);
            }

            println!("{}", pixel_color.format_color(SAMPLEPER_PIXEL))
        }
    }
    eprintln!("Done!")
}
