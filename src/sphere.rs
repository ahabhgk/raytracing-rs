use crate::{
    hit::{Hit, HitRecord},
    material::Material,
    ray::Ray,
    v3,
    vec3::Point,
};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let t1 = (-half_b - discriminant.sqrt()) / a;
            let t2 = (-half_b + discriminant.sqrt()) / a;
            if t1 < t_max && t1 > t_min {
                let p = ray.at(t1);
                let n = (p - self.center) / v3!(self.radius);
                Some(HitRecord::new(ray, p, t1, n, &self.material))
            } else if t2 < t_max && t2 > t_min {
                let p = ray.at(t2);
                let n = (p - self.center) / v3!(self.radius);
                Some(HitRecord::new(ray, p, t2, n, &self.material))
            } else {
                None
            }
        } else {
            None
        }
    }
}
