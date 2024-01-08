use std::sync::Arc;

use crate::material::Scatter;

use super::hit::{Hit, HitRecord};
use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Plane {
    normal: Vec3,
    distance: f64,
    mat: Arc<dyn Scatter>,
}

impl Plane {
    pub fn new(normal: Vec3, d: f64, m: Arc<dyn Scatter>) -> Plane {
        Plane {
            normal,
            distance: d,
            mat: m,
        }
    }
}

impl Hit for Plane {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let dn = r.direction().dot(self.normal);

        if dn == 0.0 {
            return None;
        }
        let t = (self.distance - r.origin().dot(self.normal)) / dn;

        if t < t_min || t_max < t {
            return None;
        }
        let p = r.at(t);
        let mut rec = HitRecord {
            t,
            p,
            mat: self.mat.clone(),
            normal: self.normal,
            front_face: false,
        };
        rec.set_face_normal(r, self.normal);
        Some(rec)
    }
}
