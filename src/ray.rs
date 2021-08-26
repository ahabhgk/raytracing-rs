use crate::{
    v3,
    vec3::{Point, Vec3},
};

pub struct Ray<'a> {
    pub origin: &'a Point,
    pub direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: &'a Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Point {
        *self.origin + *self.direction * v3!(distance)
    }
}
