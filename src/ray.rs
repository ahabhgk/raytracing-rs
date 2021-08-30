use crate::{
    color,
    hit::Hit,
    v3,
    vec3::{Color, Point, Vec3},
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

    pub fn to_color<H>(&self, world: &H, depth: i32) -> Color
    where
        H: Hit,
    {
        if depth <= 0 {
            return color!(0, 0, 0);
        }
        if let Some(rec) = world.hit(self, 0.001, f64::INFINITY) {
            if let Some((ray_out, attenuation)) = rec.material.scatter(self, &rec) {
                return attenuation * ray_out.to_color(world, depth - 1);
            }
            return color!(0);
        }
        let unit_dir = self.direction.unit();
        let t = 0.5 * (unit_dir.y + 1.0);
        (1.0 - t) * color!(1, 1, 1) + t * color!(0.5, 0.7, 1)
    }
}
