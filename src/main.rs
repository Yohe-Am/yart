use crate::math::vec3::*;
use crate::types::materials::*;
use crate::types::math::*;
use crate::types::*;
use std::rc::Rc;

mod types;

fn main() {
    let world = random_scene();
    std::fs::write(
        "19-hello_sayonara-hi-res.ppm",
        draw(&(Box::new(world) as Box<dyn Hit>)).as_bytes(),
    )
    .unwrap();
}

fn draw(object: &Box<dyn Hit>) -> String {
    let image_width = 1200;
    let image_height = ((image_width as Num) / 1.7) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut ppm = String::with_capacity(image_width * image_height * 12 + 20);
    ppm.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let look_from = Point::new(13, 2, 3);
    let look_at = Point::new(0, 0, 0);
    let vup = Vec3::unit_y();
    let dist_to_focus = 10.0; // (look_from - look_at).magnitude();
    let aperture = 0.1;
    let aspect_ratio = image_width as Num / image_height as Num;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        aspect_ratio,
        20.0,
        aperture,
        dist_to_focus,
    );

    let mut gen = random_num_generator();

    let mut ppm = PPM::new(image_width, image_width);

    for h in (0..(image_height - 1)).rev() {
        print!("\rScanlines remaining: {}\n", h);
        for w in 0..image_width {
            let mut pixel = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = ((w as Num) + gen()) / (image_width - 1) as Num;
                let v = ((h as Num) + gen()) / (image_height - 1) as Num;
                pixel = pixel + send_ray(object, camera.get_ray(u, v), max_depth);
            }
            ppm.set_pixel(h, w, pixel);

            // println!("h: {:?} - w: {:?} - u: {:?} - v: {:?}", h, w, u, v);
        }
    }

    println!("Done");
    ppm.print(samples_per_pixel)
}

struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}
impl PPM {
    fn new(width: usize, height: usize) -> PPM {
        PPM {
            width,
            height,
            pixels: Vec::with_capacity(width * height),
        }
    }
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x * y] = color;
    }
    fn print(self, samples_per_pixel: usize) -> String {
        use std::fmt::Write;
        let mut output = String::new();
        write!(output, "P3\n{} {}\n255\n", self.width, self.height).unwrap();
        output.reserve(self.width * self.height * 12);
        for pixel in self.pixels {
            let scale = 1.0 / samples_per_pixel as f64;
            let r = Num::sqrt(pixel.x * scale);
            let g = Num::sqrt(pixel.y * scale);
            let b = Num::sqrt(pixel.z * scale);
            // Write the translated [0,255] value of each color component.
            write!(
                output,
                "{} {} {}\n",
                (256.0 * math::clamp_num(r, 0.0, 0.999)) as i32,
                (256.0 * math::clamp_num(g, 0.0, 0.999)) as i32,
                (256.0 * math::clamp_num(b, 0.0, 0.999)) as i32,
            ).unwrap();
        }
        output
    }
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

fn random_scene() -> HittablesList {
    let mut world = HittablesList::new();
    world.push(Rc::new(Sphere {
        center: Vec3::new(0, -1000, 0),
        radius: 1000.0,
        material: Rc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    }));

    let mut gen = random_num_generator();
    let mut gen_range = random_num_generator_rng();
    let mut gen_vec = random_vec3_generator();
    let mut gen_vec_range = random_vec3_generator_rng();

    // let i = 1i32;
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = gen();
            let center = Point::new(a as Num + gen() * 0.9, 0.2, b as Num + 0.9 * gen());
            if (center - Vec3::new(4, 0.2, 0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = gen_vec() * gen_vec();

                    world.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(Lambertian { albedo }),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = gen_vec_range(0.5, 1.0);
                    let fuzz = gen_range(0.0, 0.5);
                    world.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    // glass

                    world.push(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Rc::new(Dielectric {
                            refraction_index: 1.5,
                        }),
                    }));
                }
            }
        }
    }

    world.push(Rc::new(Sphere {
        center: Vec3::new(-4, 1, 0),
        radius: 1.0,
        material: Rc::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    }));
    world.push(Rc::new(Sphere {
        center: Vec3::new(4, 1, 0),
        radius: 1.0,
        material: Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    }));
    // world.push(Rc::new(Sphere {
    //     center: Vec3::new(1, 0, -1),
    //     radius: 0.5,
    //     material: Rc::new(Dielectric {
    //         refraction_index: 1.5,
    //     }),
    // }));
    world.push(Rc::new(Sphere {
        center: Vec3::new(0, 1, 0),
        radius: 1.0,
        material: Rc::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));

    world
}
