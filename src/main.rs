use std::env;
use std::io;

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

mod demo;

type Demo = Vec<Box<dyn Fn() -> io::Result<()>>>;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("{}", "USAGE: cargo run {number}");
    }

    let demo: Demo = vec![
        Box::new(demo::demo00::run),
        Box::new(demo::demo01::run),
        Box::new(demo::demo02::run),
        Box::new(demo::demo03::run),
        Box::new(demo::demo04::run),
        Box::new(demo::demo05::run),
        Box::new(demo::demo06::run),
        Box::new(demo::demo07::run),
        Box::new(demo::demo08::run),
        Box::new(demo::demo09::run),
        Box::new(demo::demo10::run),
        Box::new(demo::demo11::run),
        Box::new(demo::demo12::run),
        Box::new(demo::demo13::run),
        Box::new(demo::demo14::run),
        Box::new(demo::demo15::run),
        Box::new(demo::demo16::run),
        Box::new(demo::demo17::run),
        Box::new(demo::demo18::run),
    ];

    let length = demo.len();

    // Run all demo
    if args[1] == "*" {
        for run in demo.iter() {
            run().unwrap();
        }
    }

    // Run specified demo
    if let Ok(n) = args[1].parse::<usize>() {
        if n < length {
            demo[n]().unwrap();
        } else {
            panic!("Demo{} is not existed", n);
        }
    } else {
        panic!("Expected a number");
    }
}
