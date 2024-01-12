use std::mem::swap;

use crate::{ray::Ray, vec::Point3};
#[derive(Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Aabb {
        let x = Interval::new(f64::min(a.x(), b.x()), f64::max(a.x(), b.x()));
        let y = Interval::new(f64::min(a.y(), b.y()), f64::max(a.y(), b.y()));
        let z = Interval::new(f64::min(a.z(), b.z()), f64::max(a.z(), b.z()));

        Aabb { x, y, z }
    }

    pub fn axis(&self, n: usize) -> Interval {
        if n == 1 {
            self.y.clone()
        } else if n == 2 {
            self.z.clone()
        } else {
            self.x.clone()
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        for i in 0..3 {
            let invD = 1.0 / r.direction()[i];
            let orig = r.origin()[i];

            let mut t0 = (self.axis(i).min - orig) * invD;
            let mut t1 = (self.axis(i).max - orig) * invD;
            if invD < 0.0 {
                swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn from_two_aabbs(box0: Aabb, box1: Aabb) -> Aabb {
        Aabb {
            x: Interval::from_two_intervals(box0.x, box1.x),
            y: Interval::from_two_intervals(box0.y, box1.y),
            z: Interval::from_two_intervals(box0.z, box1.z),
        }
    }
}
#[derive(Clone)]
pub struct Interval {
    pub max: f64,
    pub min: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { max, min }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn expand(&mut self, delta: f64) {
        let padding = delta / 2.0;
        self.min -= padding;
        self.max += padding;
    }

    pub fn from_two_intervals(a: Interval, b: Interval) -> Interval {
        Interval {
            max: f64::max(a.max, b.max),
            min: f64::min(a.min, b.min),
        }
    }
}
