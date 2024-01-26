use rand::Rng;
use std::arch::x86_64::{__m128, _mm_extract_ps, _mm_set_ps};
use std::fmt;
use std::fmt::Display;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Sub, SubAssign,
};

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self[0]
    }
    pub fn y(self) -> f64 {
        self[1]
    }
    pub fn z(self) -> f64 {
        self[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn to_reg(&self) -> __m128 {
        unsafe { _mm_set_ps(self.x() as f32, self.y() as f32, self.z() as f32, 0.0) }
    }

    pub fn from_reg(r: __m128) -> Vec3 {
        let e0 = unsafe { f64::from_bits(_mm_extract_ps(r, 1) as u64) };
        let e1 = unsafe { f64::from_bits(_mm_extract_ps(r, 2) as u64) };
        let e2 = unsafe { f64::from_bits(_mm_extract_ps(r, 3) as u64) };

        Vec3 { e: [e0, e1, e2] }
    }
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }
    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            e: [
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
            ],
        }
    }
    pub fn radnom_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::radnom_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }
    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
    }
    pub fn reflection(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
    pub fn format_color(self, sampleper_pixel: u64) -> String {
        let ir = (256.0 * (self[0] / sampleper_pixel as f64).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self[1] / sampleper_pixel as f64).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self[2] / sampleper_pixel as f64).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self * rhs[0], self * rhs[1], self * rhs[2]],
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}
