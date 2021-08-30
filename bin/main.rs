use std::{
    fs,
    ops::Range,
    sync::{mpsc::channel, Arc, Mutex},
    time::Instant,
};
use threadpool::ThreadPool;

use raytracing_rs::{camera::Camera, color, hit::HitList, point, random::Random, v3};

// Image
const DIST: &str = "dist/image.ppm";
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: usize = 1200;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

fn main() {
    // World
    let world = HitList::random_scene();
    let world = Arc::new(world);

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
    let camera = Arc::new(camera);

    // Render
    let pool = ThreadPool::new(12);
    let (sender, receiver) = channel::<(Vec<String>, Range<usize>)>();

    let header = format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut body = vec![String::new(); IMAGE_HEIGHT * IMAGE_WIDTH];
    let remaining = Arc::new(Mutex::new(IMAGE_HEIGHT));

    for j in (0..IMAGE_HEIGHT).rev() {
        let world = world.clone();
        let camera = camera.clone();
        let sender = sender.clone();
        let remaining = remaining.clone();

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

            let mut remaining = remaining.lock().unwrap();
            *remaining -= 1;
            eprintln!("\rScanlines remaining: {} ", remaining);
        };
        pool.execute(task);
    }
    drop(sender);

    let start_time = Instant::now();
    for (colors, range) in receiver.into_iter() {
        body.splice(range, colors);
    }
    eprintln!("tracking completed: {:?}", start_time.elapsed());

    let contents = format!("{}\n{}\n", header, body.join("\n"));
    fs::write(DIST, contents).expect("write file failed");
}
