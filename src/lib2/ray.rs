use std::arch::x86_64::{__m128, _mm_add_ps, _mm_mul_ps, _mm_set1_ps, _mm_set_ps};

use super::vec::{Point3, Vec3};

// pub struct RegRay {
//     orig: __m128,
//     dir: __m128,
// }

// impl RegRay {
//     pub fn new(origin: (f32, f32, f32), direction: (f32, f32, f32)) -> RegRay {
//         RegRay {
//             orig: unsafe { _mm_set_ps(origin.0, origin.1, origin.2, 0.0) },
//             dir: unsafe { _mm_set_ps(direction.0, direction.1, direction.2, 0.0) },
//         }
//     }
//     pub fn origin(&self) -> __m128 {
//         self.orig
//     }

//     pub fn direction(&self) -> __m128 {
//         self.dir
//     }
// }
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
    pub fn reg_at(&self, t: f64) -> Point3 {
        unsafe {
            Vec3::from_reg(_mm_add_ps(
                self.orig.to_reg(),
                _mm_mul_ps(_mm_set1_ps(t as f32), self.dir.to_reg()),
            ))
        }
    }
}
