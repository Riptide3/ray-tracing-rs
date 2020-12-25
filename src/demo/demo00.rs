use std::fs;
use std::io;

const FILENAME: &str = "pic/00.ppm";

pub fn run() -> io::Result<()> {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    let part0 = format!("P3\n{} {}\n255\n", image_width, image_height);

    let mut contents = part0;

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

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            let content = format!("{} {} {}\n", ir, ig, ib);

            contents.push_str(&content);
        }
    }

    fs::write(FILENAME, contents.as_bytes())?;
    eprintln!("\nDone.");
    Ok(())
}
