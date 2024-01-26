use std::arch::x86_64::{
    __m128, _mm_div_ps, _mm_dp_ps, _mm_extract_ps, _mm_mul_ps, _mm_set1_ps, _mm_set_ps, _mm_sub_ps,
};
use std::sync::Arc;

use crate::lib2::hit::RegHit;
use crate::lib2::material::Material;

use super::hit::{Hit, HitRecord};
use super::ray::Ray;
use super::vec::{Point3, Vec3};
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(cen: Point3, rad: f64, m: Arc<dyn Material>) -> Sphere {
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

impl RegHit for Sphere {
    fn reg_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = unsafe { Vec3::from_reg(_mm_sub_ps(r.origin().to_reg(), self.center.to_reg())) };
        let a = unsafe {
            f64::from_bits(_mm_extract_ps::<1>(_mm_dp_ps::<3>(
                r.direction().to_reg(),
                r.direction().to_reg(),
            )) as u64)
        };
        let half_b = unsafe {
            f64::from_bits(
                _mm_extract_ps::<1>(_mm_dp_ps::<3>(oc.to_reg(), r.direction().to_reg())) as u64,
            )
        };
        let c = unsafe {
            _mm_extract_ps(_mm_dp_ps(oc.to_reg(), oc.to_reg(), 3), 1) as f64
                - self.radius * self.radius
        };
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

        let p = r.reg_at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            mat: self.mat.clone(),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let outward_normal = unsafe {
            Vec3::from_reg(_mm_div_ps(
                _mm_sub_ps(rec.p.to_reg(), self.center.to_reg()),
                _mm_set1_ps(self.radius as f32),
            ))
        };
        rec.reg_set_face_normal(r, outward_normal);
        Some(rec)
    }
}
