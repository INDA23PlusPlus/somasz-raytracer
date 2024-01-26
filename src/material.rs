use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::{Color, Vec3};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self) -> Color;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::radnom_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }

    fn emitted(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal { albedo: a }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refelcted = r_in.direction().reflection(rec.normal).normalized();
        let scattered = Ray::new(rec.p, refelcted);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }

    fn emitted(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct DiffuseLight {
    albedo: Color,
}

impl DiffuseLight {
    pub fn new(a: Color) -> DiffuseLight {
        DiffuseLight { albedo: a }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self) -> Color {
        self.albedo
    }
}
