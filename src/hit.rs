use std::hint;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::material::Material;

use super::ray::Ray;
use super::vec::{Point3, Vec3};

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
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<Aabb>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: Aabb,
}
impl HittableList {
    pub fn new(obj: Vec<Box<dyn Hittable>>, bbox: Aabb) -> HittableList {
        HittableList { objects: obj, bbox }
    }
    pub fn push(&mut self, object: Box<dyn Hittable>) {
        let o_bbox = object.bounding_box().unwrap();
        self.objects.push(object);
        self.bbox = Aabb::from_two_aabbs(o_bbox, self.bbox.clone())
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bbox.clone())
    }
}
