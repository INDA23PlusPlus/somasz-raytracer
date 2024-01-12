use crate::hit::{Hittable, HittableList};
use crate::vec::Color;

use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    background: Color,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        background: Color,
    ) -> Camera {
        // const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const FOCAL_LENGTH: f64 = 1.0;

        let theta = std::f64::consts::PI / 180.0 * vfov;

        let viewport_hieght = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_hieght;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(cw).normalized();
        let cv = cw.cross(cu);

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = viewport_width * cu;
        let vertical = viewport_hieght * cv;
        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - cw;
        Camera {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical,
            background,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }

    pub fn ray_color(&self, r: &Ray, world: &HittableList, depth: u64) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
            let color_from_emission = rec.mat.emitted();
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                let color_from_scatter = attenuation * self.ray_color(&scattered, world, depth - 1);
                color_from_emission + color_from_scatter
            } else {
                color_from_emission
            }
        } else {
            self.background
        }
    }
}
