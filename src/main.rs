mod camera;
mod helpers;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::helpers::random;
use crate::hit::{Hit, HitList};
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Vec3};

fn ray_color<H>(ray: &Ray, world: &H, depth: i32) -> Color
where
    H: Hit,
{
    if depth <= 0 {
        return color!(0, 0, 0);
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((ray_out, attenuation)) = rec.material.scatter(ray, &rec) {
            return attenuation * ray_color(&ray_out, world, depth - 1);
        }
        return color!(0);
    }
    let unit_dir = ray.direction.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * color!(1, 1, 1) + t * color!(0.5, 0.7, 1)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;
    let image_height = image_width / aspect_ratio;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HitList::new();

    let ground = Material::new_lambertian(color!(0.8, 0.8, 0));
    let center = Material::new_lambertian(color!(0.1, 0.2, 0.5));
    let left = Material::new_dielectric(1.5);
    let right = Material::new_metal(color!(0.8, 0.6, 0.2), 0.0);

    world.add(Box::new(Sphere::new(point!(0, -100.5, -1), 100.0, ground)));
    world.add(Box::new(Sphere::new(point!(0, 0, -1), 0.5, center)));
    world.add(Box::new(Sphere::new(point!(-1, 0, -1), 0.5, left.clone())));
    world.add(Box::new(Sphere::new(point!(-1, 0, -1), -0.45, left)));
    world.add(Box::new(Sphere::new(point!(1, 0, -1), 0.5, right)));

    // Camera
    let camera = Camera::new(
        point!(-2, 2, 1),
        point!(0, 0, -1),
        v3!(0, 1, 0),
        20.0,
        aspect_ratio,
    );

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..(image_height as i32)).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..(image_width as i32) {
            let mut pixel_color = color!(0, 0, 0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1.0);
                let v = (j as f64 + random::<f64>()) / (image_height - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            pixel_color.write(samples_per_pixel);
        }
    }
}
