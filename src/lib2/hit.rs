use std::arch::x86_64::{__m128, _mm_cmplt_ps, _mm_dp_ps, _mm_extract_ps, _mm_mul_ps, _mm_set1_ps};
use std::sync::Arc;

use crate::lib2::material::Material;

use super::ray::Ray;
use super::vec::{Point3, Vec3};

// pub struct RegHitRecord {
//     pub p: __m128,
//     pub normal: __m128,
//     pub mat: Arc<dyn Material>,
//     pub t: f32,
//     pub front_face: bool,
// }

// impl RegHitRecord {
//     pub fn reg_set_face_normal(&mut self, ray: &RegRay, outward_normal: __m128) -> () {
//         self.front_face = unsafe {
//             _mm_extract_ps(
//                 _mm_cmplt_ps(
//                     _mm_dp_ps(ray.direction(), outward_normal, 3),
//                     _mm_set1_ps(0.0),
//                 ),
//                 1,
//             ) < 0
//         };
//         self.normal = if self.front_face {
//             outward_normal
//         } else {
//             unsafe { _mm_mul_ps(_mm_set1_ps(-1.0), outward_normal) }
//         };
//     }
// }

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) -> () {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
    pub fn reg_set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) -> () {
        self.front_face = unsafe {
            _mm_extract_ps(
                _mm_cmplt_ps(
                    _mm_dp_ps(ray.direction().to_reg(), outward_normal.to_reg(), 3),
                    _mm_set1_ps(0.0),
                ),
                1,
            ) < 0
        };
        self.normal = if self.front_face {
            outward_normal
        } else {
            unsafe { Vec3::from_reg(_mm_mul_ps(_mm_set1_ps(-1.0), outward_normal.to_reg())) }
        };
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
pub trait RegHit: Send + Sync {
    fn reg_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;
        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}

pub type RegWorld = Vec<Box<dyn RegHit>>;

impl RegHit for RegWorld {
    fn reg_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;
        for object in self {
            if let Some(rec) = object.reg_hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}
