use std::{
    ops::Range,
    sync::{mpsc::channel, Arc, Mutex},
};

use criterion::{criterion_group, criterion_main, Criterion};
use rayon::iter::{ParallelBridge, ParallelIterator};
use raytracing_rs::{camera::Camera, color, hit::HitList, point, random::Random, v3};
use threadpool::ThreadPool;

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: usize = 90;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: i32 = 10;
const MAX_DEPTH: i32 = 10;

struct RayTracing {
    world: HitList,
    camera: Camera,
}

impl RayTracing {
    pub fn new() -> Self {
        // World
        let world = HitList::random_scene();

        // Camera
        let look_from = point!(13, 2, 3);
        let look_at = point!(0, 0, 0);
        let up = v3!(0, 1, 0);
        let vertical = 20.0;
        let aperture = 0.1;
        let focus_distance = 10.0;

        let camera = Camera::new(
            look_from,
            look_at,
            up,
            vertical,
            ASPECT_RATIO,
            aperture,
            focus_distance,
        );

        Self { world, camera }
    }

    pub fn single_thread_render(self) -> String {
        let header = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
        let mut body = Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH);

        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                let mut pixel = color!(0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + f64::random()) / (IMAGE_WIDTH as f64 - 1.0);
                    let v = (j as f64 + f64::random()) / (IMAGE_HEIGHT as f64 - 1.0);
                    let ray = self.camera.get_ray(u, v);
                    pixel += ray.to_color(&self.world, MAX_DEPTH);
                }
                body.push(pixel.to_rgb_string(SAMPLES_PER_PIXEL));
            }
        }

        let contents = format!("{}\n{}\n", header, body.join("\n"));
        contents
    }

    pub fn multi_thread_render(self) -> String {
        let pool = ThreadPool::new(12);
        let (sender, receiver) = channel::<(Vec<String>, Range<usize>)>();
        let world = Arc::new(self.world);
        let camera = Arc::new(self.camera);

        let header = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
        let mut body = vec![String::new(); IMAGE_HEIGHT * IMAGE_WIDTH];

        for j in (0..IMAGE_HEIGHT).rev() {
            let world = world.clone();
            let camera = camera.clone();
            let sender = sender.clone();

            let task = move || {
                let mut colors = Vec::with_capacity(IMAGE_WIDTH);
                for i in 0..IMAGE_WIDTH {
                    let mut pixel = color!(0);
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f64 + f64::random()) / (IMAGE_WIDTH as f64 - 1.0);
                        let v = (j as f64 + f64::random()) / (IMAGE_HEIGHT as f64 - 1.0);
                        let ray = camera.get_ray(u, v);
                        pixel += ray.to_color(&*world, MAX_DEPTH);
                    }
                    colors.push(pixel.to_rgb_string(SAMPLES_PER_PIXEL));
                }
                let start = (IMAGE_HEIGHT - 1 - j) * IMAGE_WIDTH;
                let end = start + IMAGE_WIDTH;
                let range = start..end;
                sender.send((colors, range)).expect("ray tracing failed");
            };
            pool.execute(task);
        }
        drop(sender);

        for (colors, range) in receiver.into_iter() {
            body.splice(range, colors);
        }

        let contents = format!("{}\n{}\n", header, body.join("\n"));
        contents
    }

    pub fn multi_thread_per_ray_render(self) -> String {
        let pool = ThreadPool::new(12);
        let (sender, receiver) = channel::<(String, usize)>();
        let world = Arc::new(self.world);
        let camera = Arc::new(self.camera);

        let header = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
        let mut body = vec![String::new(); IMAGE_HEIGHT * IMAGE_WIDTH];

        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                let world = world.clone();
                let camera = camera.clone();
                let sender = sender.clone();

                let task = move || {
                    let mut pixel = color!(0);
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = (i as f64 + f64::random()) / (IMAGE_WIDTH as f64 - 1.0);
                        let v = (j as f64 + f64::random()) / (IMAGE_HEIGHT as f64 - 1.0);
                        let ray = camera.get_ray(u, v);
                        pixel += ray.to_color(&*world, MAX_DEPTH);
                    }
                    let rgb = pixel.to_rgb_string(SAMPLES_PER_PIXEL);
                    let index = (IMAGE_HEIGHT - 1 - j) * IMAGE_WIDTH + i;
                    sender.send((rgb, index)).expect("ray tracing failed");
                };
                pool.execute(task);
            }
        }
        drop(sender);

        for (rgb, i) in receiver.into_iter() {
            body.insert(i, rgb);
        }

        let contents = format!("{}\n{}\n", header, body.join("\n"));
        contents
    }

    pub fn rayon_parallel_render(self) -> String {
        let header = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
        let body = Mutex::new(Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH));

        (0..IMAGE_HEIGHT).rev().par_bridge().for_each(|j| {
            for i in 0..IMAGE_WIDTH {
                let mut pixel = color!(0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + f64::random()) / (IMAGE_WIDTH as f64 - 1.0);
                    let v = (j as f64 + f64::random()) / (IMAGE_HEIGHT as f64 - 1.0);
                    let ray = self.camera.get_ray(u, v);
                    pixel += ray.to_color(&self.world, MAX_DEPTH);
                }
                body.lock()
                    .unwrap()
                    .push(pixel.to_rgb_string(SAMPLES_PER_PIXEL));
            }
        });

        let contents = format!("{}\n{}\n", header, body.lock().unwrap().join("\n"));
        contents
    }

    pub fn rayon_parallel_per_ray_render(self) -> String {
        let header = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

        let pairs = (0..IMAGE_HEIGHT)
            .rev()
            .flat_map(|j| (0..IMAGE_WIDTH).map(move |i| (j, i)));

        let body = pairs
            .par_bridge()
            .map(|(j, i)| {
                let mut pixel = color!(0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + f64::random()) / (IMAGE_WIDTH as f64 - 1.0);
                    let v = (j as f64 + f64::random()) / (IMAGE_HEIGHT as f64 - 1.0);
                    let ray = self.camera.get_ray(u, v);
                    pixel += ray.to_color(&self.world, MAX_DEPTH);
                }
                pixel.to_rgb_string(SAMPLES_PER_PIXEL)
            })
            .collect::<Vec<String>>();

        let contents = format!("{}\n{}\n", header, body.join("\n"));
        contents
    }
}

pub fn single_thread(c: &mut Criterion) {
    c.bench_function("single thread", |b| {
        b.iter(|| RayTracing::new().single_thread_render())
    });
}

pub fn multi_thread(c: &mut Criterion) {
    c.bench_function("multi thread", |b| {
        b.iter(|| RayTracing::new().multi_thread_render())
    });
}

pub fn multi_thread_per_ray(c: &mut Criterion) {
    c.bench_function("multi thread per ray", |b| {
        b.iter(|| RayTracing::new().multi_thread_per_ray_render())
    });
}

pub fn rayon_parallel(c: &mut Criterion) {
    c.bench_function("rayon parallel", |b| {
        b.iter(|| RayTracing::new().rayon_parallel_render())
    });
}

pub fn rayon_parallel_per_ray(c: &mut Criterion) {
    c.bench_function("rayon parallel per ray", |b| {
        b.iter(|| RayTracing::new().rayon_parallel_per_ray_render())
    });
}

criterion_group!(
    benches,
    single_thread,
    multi_thread,
    multi_thread_per_ray,
    rayon_parallel,
    rayon_parallel_per_ray
);
criterion_main!(benches);
