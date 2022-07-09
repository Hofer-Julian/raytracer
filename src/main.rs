use rayon::iter::ParallelIterator;
use std::{
    fs::File,
    io::{stdout, Write},
    sync::Arc,
};

mod camera;
mod color;
mod hit;
mod material;
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
use rayon::iter::IntoParallelIterator;
use sphere::Sphere;
use vec3::Vec3;

use crate::material::Lambertian;

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.point + rec.normal + Vec3::random_in_unit_sphere().normalized();
        let r = Ray::new(rec.point, target - rec.point);
        0.5 * ray_color(&r, world, depth - 1)
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
    const MAX_DEPTH: u64 = 50;

    // World
    let mut world = World::new();
    let lambertian = Arc::new(Lambertian::new(Color::new(0.8, 0.3, 0.3)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        lambertian.clone(),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        lambertian.clone(),
    )));

    // Camera
    let camera = Camera::new();

    let filename = "image.ppm";
    match std::fs::remove_file(filename) {
        Ok(_) => (),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (), // File doesn't exist, so we can ignore this error
        Err(e) => panic!("{}", e),
    }
    let mut file = File::create(filename).unwrap();

    writeln!(file, "P3").unwrap();
    writeln!(file, "{IMAGE_WIDTH} {IMAGE_HEIGHT}").unwrap();
    writeln!(file, "255").unwrap();

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {:3}", j);
        stdout().flush().unwrap();

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            writeln!(file, "{}", pixel_color.format_color(SAMPLES_PER_PIXEL)).unwrap();
        }
    }
    println!();
    println!("Done");
}
