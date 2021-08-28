use crate::{
    color,
    material::Material,
    point,
    random::Random,
    ray::Ray,
    sphere::Sphere,
    vec3::{Color, Point, Vec3},
};

pub struct HitRecord<'a> {
    pub point: Point,
    pub t: f64,
    pub normal: Vec3,
    pub material: &'a Material,
    pub is_front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        ray: &Ray,
        point: Point,
        t: f64,
        outward_normal: Vec3,
        material: &'a Material,
    ) -> Self {
        let is_front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            t,
            normal,
            material,
            is_front_face,
        }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitList {
    objects: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hit>) {
        self.objects.push(object);
    }

    pub fn random_scene() -> Self {
        let mut scene = Self::new();

        let ground_material = Material::new_lambertian(color!(0.5, 0.5, 0.5));
        let ground = Sphere::new(point!(0, -1000, 0), 1000.0, ground_material);
        scene.add(ground);

        for a in -11..11 {
            for b in -11..11 {
                let a = f64::from(a);
                let b = f64::from(b);

                let choose_mat = f64::random();
                let center = point!(a + 0.9 * f64::random(), 0.2, b + 0.9 * f64::random());

                if (center - point!(4, 0.2, 0)).len() > 0.9 {
                    let sphere_material = if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        Material::new_lambertian(albedo)
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random_in(color!(0.5), color!(1));
                        let fuzz = f64::random_in(0.0, 0.5);
                        Material::new_metal(albedo, fuzz)
                    } else {
                        Material::new_dielectric(1.5)
                    };
                    let sphere = Sphere::new(center, 0.2, sphere_material);
                    scene.add(sphere);
                }
            }
        }

        let dielectric = Material::new_dielectric(1.5);
        scene.add(Sphere::new(point!(0, 1, 0), 1.0, dielectric));

        let lambertian = Material::new_lambertian(color!(0.4, 0.2, 0.1));
        scene.add(Sphere::new(point!(-4, 1, 0), 1.0, lambertian));

        let metal = Material::new_metal(color!(0.7, 0.6, 0.5), 0.0);
        scene.add(Sphere::new(point!(4, 1, 0), 1.0, metal));

        scene
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
