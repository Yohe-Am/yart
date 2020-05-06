use crate::math::Num;

mod math;

fn main() {
    println!("{}", hello_ray());
}

fn hello_ray() -> String {
    let mut output = String::with_capacity(661_886);
    let image_width = 256;
    let image_height = 256;
    output.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height)[..]);

    for j in (0..(image_height - 1)).rev() {
        // println!("\n Scanlines remaining: {}", j);
        for i in 0..image_width {
            let r = i as Num / (image_width - 1) as Num;
            let g = j as Num / (image_height - 1) as Num;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            output.push_str(&format!("{} {} {}\n", ir, ig, ib)[..]);
        }
    }
    output
}
