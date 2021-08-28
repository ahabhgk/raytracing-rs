use crate::{
    color,
    hit::HitRecord,
    random::Random,
    ray::Ray,
    vec3::{Color, Vec3},
};
use std::rc::Rc;

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refraction_index: f64 },
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Rc<Self> {
        Rc::new(Self::Lambertian { albedo })
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Rc<Self> {
        Rc::new(Self::Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        })
    }

    pub fn new_dielectric(refraction_index: f64) -> Rc<Self> {
        Rc::new(Self::Dielectric { refraction_index })
    }

    pub fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        match *self {
            Self::Lambertian { albedo } => Self::scatter_lambertion(record, albedo),
            Self::Metal { albedo, fuzz } => Self::scatter_metal(ray_in, record, albedo, fuzz),
            Self::Dielectric { refraction_index } => {
                Self::scatter_dielectric(ray_in, record, refraction_index)
            }
        }
    }

    fn scatter_lambertion(record: &HitRecord, albedo: Color) -> Option<(Ray, Color)> {
        let mut scatter_direction = record.normal + Vec3::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal
        }
        Some((Ray::new(record.point, scatter_direction), albedo))
    }

    fn scatter_metal(
        ray_in: &Ray,
        record: &HitRecord,
        albedo: Color,
        fuzz: f64,
    ) -> Option<(Ray, Color)> {
        let reflected = ray_in.direction.unit().reflect(&record.normal);
        let scattered = Ray::new(
            record.point,
            reflected + fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.direction.dot(&record.normal) > 0.0 {
            Some((scattered, albedo))
        } else {
            None
        }
    }

    fn scatter_dielectric(
        ray_in: &Ray,
        record: &HitRecord,
        refraction_index: f64,
    ) -> Option<(Ray, Color)> {
        let attenuation = color!(1);
        let refraction_ratio = if record.is_front_face {
            1.0 / refraction_index
        } else {
            refraction_index
        };

        let unit_direction = ray_in.direction.unit();
        let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Material::reflectance(cos_theta, refraction_ratio) > f64::random()
        {
            unit_direction.reflect(&record.normal)
        } else {
            unit_direction.refract(&record.normal, refraction_ratio)
        };

        Some((Ray::new(record.point, direction), attenuation))
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
