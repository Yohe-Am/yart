use crate::math::{Num, Vector3};
use std::fs;

mod math;

fn main() {
    fs::write("hello_ray.ppm", hello_ray().as_bytes()).unwrap();
}

fn hello_ray() -> String {
    let mut output = String::with_capacity(661_886);
    let image_width = 256;
    let image_height = 256;
    output.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    for j in (0..(image_height - 1)).rev() {
        print!("\rScanlines remaining: {}\n", j);
        for i in 0..image_width {
            let pixel = Color::new(
                i as Num / (image_width - 1) as Num,
                j as Num / (image_height - 1) as Num,
                0.25,
            );
            output.push_str(pixel.ppm_fmt().as_str());
        }
    }
    println!("Done");
    output
}

type Color = Vector3;

// use std::fmt::{Display, Formatter, Result};
impl Color {
    fn ppm_fmt(&self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.x) as i32,
            (255.999 * self.y) as i32,
            (255.999 * self.z) as i32
        )
    }
}

type Point = Vector3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    fn at(self, t: Num) -> Point {
        self.origin + (self.direction * t)
    }
}
