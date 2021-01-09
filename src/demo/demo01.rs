use std::fs::File;
use std::io;
use std::io::Write;

use crate::vec3::Color;

const FILENAME: &str = "pic/01.ppm";

pub fn run() -> io::Result<()> {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut f = File::create(FILENAME)?;
    f.write_all(part0.as_bytes())?;

    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", row);
        for col in 0..image_width {
            let row = row as f64;
            let col = col as f64;
            let width = (image_width - 1) as f64;
            let height = (image_height - 1) as f64;

            let r = col / width;
            let g = row / height;
            let b = 0.25;

            let pixel_color = Color { 0: r, 1: g, 2: b };
            pixel_color.write_color(&mut f)?;
        }
    }

    eprintln!("\nDone.");
    Ok(())
}
