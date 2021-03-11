use crate::materials::*;
use crate::math::vec3::*;
use crate::math::*;
use crate::types::*;
use std::sync::mpsc;
use std::sync::Arc;

pub mod materials;
pub mod math;
pub mod types;

mod threads;

fn main() {
    let world = random_scene();
    let world = Arc::new(world); // as Arc<dyn Hit + Send + Sync>;
                                 // let ref world = make_threadsafe_hittable(Box::new(random_scene()));
    std::fs::write("21-hello_hello.ppm", draw(world).as_bytes()).unwrap();
}

fn draw(object_ptr: HittablePtr) -> String {
    let image_width = 1366;
    let image_height = (((image_width as Num) * 9.0) / 16.0) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let thread_count = 4;

    let camera_ptr = {
        let look_from = Point::new(4, 2, 3);
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
            90.0,
            aperture,
            dist_to_focus,
        );
        Arc::new(camera)
    };

    let (sender, reciever) = mpsc::channel();

    let pool = crate::threads::ThreadPool::new(thread_count);

    for h in (0..(image_height - 1)).rev() {
        // reversed: top to bottom
        let camera_ptr = camera_ptr.clone();
        let object_ptr = object_ptr.clone();
        let sender = sender.clone();
        // a single thread for a single scan line
        pool.execute(move || {
            for w in 0..image_width {
                // right to left
                let mut pixel = Color::zero();
                for _ in 0..samples_per_pixel {
                    let object = object_ptr.clone();
                    let u = ((w as Num) + random_num()) / (image_width - 1) as Num;
                    let v = ((h as Num) + random_num()) / (image_height - 1) as Num;
                    pixel = pixel + send_ray(object, camera_ptr.get_ray(u, v), max_depth);
                }
                sender.send((h, w, pixel)).unwrap();
                // println!("h: {:?} - w: {:?} - u: {:?} - v: {:?}", h, w, u, v);
            }
        });
    }
    let mut ppm = PPM::new(image_width, image_height);
    let mut scanlines_remaining = image_height;
    loop {
        let (h, w, pixel) = reciever.recv().unwrap();
        ppm.set_pixel(w, h, pixel);
        if h < scanlines_remaining {
            print!("\rScanlines remaining: {}\n", h);
            scanlines_remaining = h;
        }
        if h == 0 && w == (image_width - 1) {
            // if on last pixel
            break;
        }
    }
    ppm.print(samples_per_pixel)
}

fn send_ray(hittable: HittablePtr, ray: Ray, depth: i32) -> Color {
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
    world.push(Arc::new(Sphere {
        center: Vec3::new(0, -1000.0, 0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    }));

    // let i = 1i32;
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_num();
            let center = Point::new(
                a as Num + random_num() * 0.9,
                0.2,
                b as Num + 0.9 * random_num(),
            );
            if (center - Vec3::new(4, 0.2, 0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = random_vec3() * random_vec3();

                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Lambertian { albedo }),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec3_rng(0.5, 1.0);
                    let fuzz = random_num_rng(0.0, 0.5);
                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    // glass

                    world.push(Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Arc::new(Dielectric {
                            refraction_index: 1.5,
                        }),
                    }));
                }
            }
        }
    }

    world.push(Arc::new(Sphere {
        center: Vec3::new(-4, 1, 0),
        radius: 1.0,
        material: Arc::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    }));
    world.push(Arc::new(Sphere {
        center: Vec3::new(4, 1, 0),
        radius: 1.0,
        material: Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    }));
    // world.push(Arc::new(Sphere {
    //     center: Vec3::new(1, 0, -1),
    //     radius: 0.5,
    //     material: Arc::new(Dielectric {
    //         refraction_index: 1.5,
    //     }),
    // }));
    world.push(Arc::new(Sphere {
        center: Vec3::new(0, 1, 0),
        radius: 1.0,
        material: Arc::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));

    world
}

// #[derive(Debug)]
struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}
use std::fmt;
impl fmt::Debug for PPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PPM {{ width: {}, height: {} }}",
            self.width, self.height
        )
    }
}

impl PPM {
    fn new(width: usize, height: usize) -> PPM {
        let mut pixels = Vec::with_capacity(height);
        for _ in 0..(height - 1) {
            pixels.push(Vec::with_capacity(width));
        }
        PPM {
            width,
            height,
            pixels,
        }
    }
    fn set_pixel(&mut self, _w: usize, h: usize, color: Color) {
        self.pixels[h].push(color);
    }
    fn print(self, samples_per_pixel: usize) -> String {
        use std::fmt::Write;
        let mut output = String::new();
        write!(output, "P3\n{} {}\n255\n", self.width, self.height).unwrap();
        output.reserve(self.width * self.height * 12);
        for h in (0..(self.height - 1)).rev() {
            for pixel in &self.pixels[h] {
                // let pixel = self.pixels[&(h * w)];
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
                )
                .unwrap();
            }
        }
        output
    }
}
