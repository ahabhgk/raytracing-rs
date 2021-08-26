mod vec3;

use vec3::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0.25;
            let color = Color::new(r, g, b);
            color.write();
        }
    }
}
