use crate::math::vec3::*;
use crate::types::materials::*;
use crate::types::math::*;
use crate::types::*;
use std::rc::Rc;

mod types;

fn main() {
    let mut world = HittablesList::new();
    world.push(Rc::new(Sphere {
        center: Vec3::new(0, -100.5, -1),
        radius: 100.0,
        material: Rc::new(Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        }),
    }));
    world.push(Rc::new(Sphere {
        center: Vec3::new(0, 0, -1),
        radius: 0.5,
        material: Rc::new(Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        }),
    }));
    world.push(Rc::new(Sphere {
        center: Vec3::new(-1, 0, -1),
        radius: 0.5,
        material: Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3)),
    }));
    world.push(Rc::new(Sphere {
        center: Vec3::new(1, 0, -1),
        radius: 0.5,
        material: Rc::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));
    world.push(Rc::new(Sphere {
        center: Vec3::new(1, 0, -1),
        radius: -0.45,
        material: Rc::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));
    std::fs::write(
        "15-hello_lookat.ppm",
        draw(&(Box::new(world) as Box<dyn Hit>)).as_bytes(),
    )
    .unwrap();
}

fn draw(object: &Box<dyn Hit>) -> String {
    let image_width = 384;
    let image_height = ((image_width as Num) / 1.7) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut ppm = String::with_capacity(image_width * image_height * 12 + 20);
    ppm.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let aspect_ratio = image_width as Num / image_height as Num;
    let camera = Camera::new(
        Point::new(-2, 2, 1),
        Point::new(0, 0, -1),
        Vec3::unit_y(),
        aspect_ratio,
        20.0,
    );

    let mut gen = random_num_generator();

    for h in (0..(image_height - 1)).rev() {
        print!("\rScanlines remaining: {}\n", h);
        for w in 0..image_width {
            let mut pixel = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = ((w as Num) + gen()) / (image_width - 1) as Num;
                let v = ((h as Num) + gen()) / (image_height - 1) as Num;
                pixel = pixel + send_ray(object, camera.get_ray(u, v), max_depth);
            }
            write_color(&mut ppm, pixel, samples_per_pixel);

            // println!("h: {:?} - w: {:?} - u: {:?} - v: {:?}", h, w, u, v);
        }
    }

    println!("Done");
    ppm
}
fn write_color(output: &mut String, pixel: Color, samples_per_pixel: i32) {
    // Divide the color total by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = Num::sqrt(pixel.x * scale);
    let g = Num::sqrt(pixel.y * scale);
    let b = Num::sqrt(pixel.z * scale);
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

fn send_ray(hittable: &Box<dyn Hit>, ray: Ray, depth: i32) -> Color {
    if depth <= 0 {
        // no more light if at end of depth
        return Color::zero();
    }
    // TODO: check out shaodw acne
    match hittable.hit(&ray, 0.001, INFINITY) {
        // if it hits the hittable, get color
        Some(record) => match record.material.clone().scatter(ray, record) {
            Some((deflected_ray, attenuation)) => {
                attenuation * send_ray(hittable, deflected_ray, depth - 1)
            }
            None => Color::zero(),
        },
        // else, the background gradient
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            (Color::one() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
            //^ white                     ^ blue
        }
    }
}
