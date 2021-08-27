use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn lambertian(albedo: Color) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn metal(albedo: Color, fuzz: f64) -> Self {
        Self::Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    pub fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        match *self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = record.normal + Vec3::random_unit_vector();
                if scatter_direction.is_near_zero() {
                    scatter_direction = record.normal
                }
                Some((Ray::new(record.point, scatter_direction), albedo))
            }
            Self::Metal { albedo, fuzz } => {
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
        }
    }
}
