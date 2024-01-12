use crate::hit::{Hittable, HittableList};

pub struct BVHNode {
    node: (Vec<Box<dyn Hittable>>, usize, usize),
}

impl BVHNode {
    pub fn new(list: HittableList) -> BVHNode {}
}
