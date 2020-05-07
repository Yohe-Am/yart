use crate::types::math::*;
use crate::types::*;
use rand::Rng;
use std::rc::Rc;

mod types;

fn main() {
    let mut world = HittablesList::new();
    world.push(Rc::new(Sphere {
        center: Vec3::new(0, 0, -1),
        radius: 0.5,
    }));
    world.push(Rc::new(Sphere {
        center: Vec3::new(0, -100.5, -1),
        radius: 100.0,
    }));
    //rand::thread_rng().gen_range(1, 101);
    std::fs::write(
        "hello_anti-aliasing.ppm",
        draw(&(Box::new(world) as Box<dyn Hit>)).as_bytes(),
    )
    .unwrap();
}

fn draw(object: &Box<dyn Hit>) -> String {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = ((image_width as Num) / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let mut ppm = String::with_capacity(990_735);
    ppm.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let camera = Camera::standard();

    let mut gen = get_random_num_generator();

    for h in (0..(image_height - 1)).rev() {
        print!("\rScanlines remaining: {}\n", h);
        for w in 0..image_width {
            let mut pixel = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = ((w as Num) + gen()) / (image_width - 1) as Num;
                let v = ((h as Num) + gen()) / (image_height - 1) as Num;
                pixel = pixel + ray_color(object, camera.get_ray(u, v));
            }
            write_color(&mut ppm, pixel, samples_per_pixel);

            // println!("h: {:?} - w: {:?} - u: {:?} - v: {:?}", h, w, u, v);
        }
    }

    println!("Done");
    ppm
}

fn get_random_num_generator() -> impl FnMut() -> Num {
    let mut rng = rand::thread_rng();
    move || rng.gen::<f64>()
}

fn write_color(output: &mut String, pixel: Color, samples_per_pixel: i32) {
    // Divide the color total by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = pixel.x * scale;
    let g = pixel.y * scale;
    let b = pixel.z * scale;
    // Write the translated [0,255] value of each color component.
    output.push_str(
        format!(
            "{} {} {}\n",
            (256.0 * math::clamp_num(r, 0.0, 0.999)) as i32,
            (256.0 * math::clamp_num(g, 0.0, 0.999)) as i32,
            (256.0 * math::clamp_num(b, 0.0, 0.999)) as i32,
        )
        .as_str(),
    );
}

fn ray_color(hittable: &Box<dyn Hit>, ray: Ray) -> Color {
    match hittable.hit(&ray, 0.0, INFINITY) {
        Some(record) => (record.normal + Color::one()) * 0.5,
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            (Color::one() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
            //^ white                     ^ blue
        }
    }
}
