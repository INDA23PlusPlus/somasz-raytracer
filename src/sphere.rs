use std::arch::x86_64::{__m128, _mm_set_ps};
use std::sync::Arc;

use crate::material::Scatter;

use super::hit::{Hit, HitRecord};
use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct RegSphere {
    center: __m128,
    radius: f32,
    mat: Arc<dyn Scatter>,
}

impl RegSphere {
    pub fn new(cen: (f32, f32, f32), rad: f32, m: Arc<dyn Scatter>) -> RegSphere {
        RegSphere {
            center: unsafe { _mm_set_ps(cen.0, cen.1, cen.2, 0.0) },
            radius: rad,
            mat: m,
        }
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Scatter>,
}
impl Sphere {
    pub fn new(cen: Point3, rad: f64, m: Arc<dyn Scatter>) -> Sphere {
        Sphere {
            center: cen,
            radius: rad,
            mat: m,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius * self.radius;

        let discriminant = half_b.powf(2.0) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
}
