use std::{
    fs::File,
    io::{stdout, Write},
};

mod vec3;

use vec3::{Color, Point3, Vec3};

fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    let mut file = File::create("image.ppm").unwrap();

    writeln!(file, "P3").unwrap();
    writeln!(file, "{IMAGE_WIDTH} {IMAGE_HEIGHT}").unwrap();
    writeln!(file, "255").unwrap();

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.25,
            );

            writeln!(file, "{}", pixel_color.format_color()).unwrap();
        }
    }
    println!("\nDone");
}
