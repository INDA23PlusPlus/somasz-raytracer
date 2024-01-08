mod camera;
mod hit;
mod material;
mod planes;
mod ray;
mod sphere;
mod vec;

use camera::Camera;
use hit::{Hit, World};
use material::{Lambertian, Metal};
use planes::Plane;
use rand::*;
use ray::Ray;
use sphere::Sphere;
use std::io::{stderr, Write};
use std::sync::Arc;

use vec::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        //Random Diffuse
        // let target = rec.p + rec.normal + Vec3::radnom_in_unit_sphere();

        //True Labertian
        // let target = rec.p + rec.normal + Vec3::radnom_in_unit_sphere().normalized();

        //Old Method
        // let target = rec.p + Vec3::random_in_hemisphere(rec.normal);

        // let r = Ray::new(rec.p, target - rec.p);
        // 0.5 * ray_color(&r, world, depth - 1)
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();

        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    //World
    let mut world = World::new();
    let mat_ground_plane = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.5, 0.0, -1.5), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.5, 0.0, -1.5), 0.5, mat_right);
    let ground_plane = Plane::new(
        Vec3::new(0.0, 1.0, 0.0).normalized(),
        -0.5,
        mat_ground_plane,
    );
    world.push(Box::new(ground_plane));

    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    //Camera
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO,
        Color::new(0.3, 0.5, 0.9),
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
            for _ in 0..SAMPLES_PER_PIXEL {
                let radnom_v: f64 = rng.gen();
                let radnom_u: f64 = rng.gen();

                let u = ((i as f64) + radnom_u) / (IMAGE_WIDTH - 1) as f64;
                let v = ((j as f64) + radnom_v) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL))
        }
    }
    eprintln!("Done!")
}
