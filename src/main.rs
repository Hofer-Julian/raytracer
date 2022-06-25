use std::{
    fs::File,
    io::{stdout, Write},
};

mod camera;
mod color;
mod hit;
mod point3;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hit::{Hit, World};
use point3::Point3;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;

fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        let color_normal = Color::new(rec.normal.x(), rec.normal.y(), rec.normal.z());
        0.5 * (color_normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // World
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    let mut file = File::create("image.ppm").unwrap();

    writeln!(file, "P3").unwrap();
    writeln!(file, "{IMAGE_WIDTH} {IMAGE_HEIGHT}").unwrap();
    writeln!(file, "255").unwrap();

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {:3}", j);
        stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            writeln!(file, "{}", pixel_color.format_color(SAMPLES_PER_PIXEL)).unwrap();
        }
    }
    println!();
    println!("Done");
}
