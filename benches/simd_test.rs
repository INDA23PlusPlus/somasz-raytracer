use rand::*;
use somasz_raytracer::lib2::camera::{self, Camera};
use somasz_raytracer::lib2::hit::{Hit, World};
use somasz_raytracer::lib2::material::{Lambertian, Metal};
use somasz_raytracer::lib2::planes::Plane;
use somasz_raytracer::lib2::ray::Ray;
use somasz_raytracer::lib2::sisd_version;
use somasz_raytracer::lib2::sphere::Sphere;
use std::io::{stderr, Write};
use std::sync::Arc;

use somasz_raytracer::lib2::vec::{Color, Point3, Vec3};

use somasz_raytracer::lib2::hit::RegWorld;
use somasz_raytracer::lib2::material::DiffuseLight;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn simd_test(c: &mut Criterion) {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

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

    let world = black_box(world);
    let camera = black_box(Camera::new(
        Point3::new(0.0, 0.0, 1.3),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        ASPECT_RATIO,
        Color::new(0.0, 0.0, 0.0), // Color::new(0.70, 0.80, 1.00),
    ));
    let mut group = c.benchmark_group("low size");
    group.sample_size(20);
    group.bench_function("sisd_test", |b| b.iter(|| sisd_version(&world, camera)));
    group.finish();
}

criterion_group!(benches, simd_test);
criterion_main!(benches);
