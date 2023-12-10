mod ray;
mod vec;

use ray::Ray;
use std::io::{stderr, Write};
use vec::{Color, Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalized();

    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(1.0, 0.2, 0.5)
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;
    //Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * ASPECT_RATIO;
    let focal_length: f64 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining {:3}", (IMAGE_HEIGHT - j - 1));
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel = ray_color(&ray);

            println!("{}", pixel.format_color())
        }
    }
    eprintln!("Done!")
}
