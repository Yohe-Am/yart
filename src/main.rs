use crate::types::*;

mod types;

fn main() {
    std::fs::write("hello_shading.ppm", hello_ray().as_bytes()).unwrap();
}

fn hello_ray() -> String {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    let mut ppm = String::with_capacity(661_886);
    ppm.push_str(format!("P3\n{} {}\n255\n", image_width, image_height).as_str());

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.25, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::unit_z(); // FIXME: negative-z

    let sphere: Box<dyn Hit> = Box::new(Sphere {
        center: Vec3::new(0, 0, -1),
        radius: 0.5,
    });
    /*
    let sphere = Sphere {
        center: Vec3::new(0, 0, -1),
        radius: 0.5,
    };
    */

    for h in (0..(image_height - 1)).rev() {
        // print!("\rScanlines remaining: {}\n", h);
        for w in 0..image_width {
            let u = w as Num / (image_width - 1) as Num;
            let v = h as Num / (image_height - 1) as Num;

            // println!("h: {:?} - w: {:?} - u: {:?} - v: {:?}", h, w, u, v);

            let r = Ray {
                origin,
                direction: lower_left_corner + (horizontal * u) + (vertical * v),
            };
            let pixel = ray_color(&sphere, r);
            ppm.push_str(pixel.ppm_fmt().as_str());
        }
    }

    println!("Done");
    ppm
}
fn ray_color(hittable: &Box<dyn Hit>, ray: Ray) -> Color {
    return match hittable.hit(&ray, 0.0, std::f64::INFINITY) {
        Some(record) => (record.normal + Color::one()) * 0.5,
        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            (Color::one() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
            //^ white                     ^ blue
        }
    };
}