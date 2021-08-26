use crate::{
    v3,
    vec3::{Point, Vec3},
};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Point {
        self.origin + self.direction * v3!(distance)
    }
}
