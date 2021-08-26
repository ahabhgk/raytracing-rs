mod hit;
mod ray;
mod sphere;
mod vec3;

use crate::hit::{Hit, HitList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Vec3};
use std::rc::Rc;

fn ray_color<H>(ray: &Ray, world: &H) -> Color
where
    H: Hit,
{
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (rec.normal + color!(1, 1, 1));
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

    // World
    let mut world = HitList::new();
    world.add(Rc::new(Sphere::new(point!(0, 0, -1), 0.5)));
    world.add(Rc::new(Sphere::new(point!(0, -100.5, -1), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point!(0, 0, 0);
    let horizontal = v3!(viewport_width, 0, 0);
    let vertical = v3!(0, viewport_height, 0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - v3!(0, 0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..(image_height as i32)).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..(image_width as i32) {
            let u = (i as f64) / (image_width - 1.0);
            let v = (j as f64) / (image_height - 1.0);
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);
            let color = ray_color(&ray, &world);
            color.write();
        }
    }
}
