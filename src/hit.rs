use std::rc::Rc;

use crate::{
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct HitRecord {
    pub point: Point,
    pub t: f64,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(ray: &Ray, point: Point, t: f64, outward_normal: Vec3) -> Self {
        let normal = if ray.direction.dot(&outward_normal) < 0.0 {
            outward_normal
        } else {
            -outward_normal
        };
        Self { point, t, normal }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitList {
    objects: Vec<Rc<dyn Hit>>,
}

impl HitList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hit>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for HitList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                res = Some(rec);
            }
        }

        res
    }
}
