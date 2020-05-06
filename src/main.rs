use crate::types::*;

mod types;

fn main() {
    std::fs::write("hello_sphere.ppm", hello_ray().as_bytes()).unwrap();
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
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::unit_z(); // FIXME: negeative-z

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
            let pixel = ray_color(r);
            ppm.push_str(pixel.ppm_fmt().as_str());
        }
    }

    println!("Done");
    ppm
}
fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    (Color::one() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    //^ white                     ^ blue
}

fn hit_sphere(center: Point, radius: Num, ray: &Ray) -> bool {
    // (t^2 * b^2) + (2tb * (A−C)) + ((A−C) * (A−C)) − r2 = 0
    // A = origin
    // b = direction
    // t = step
    // C = sphere center

    // use quadratic equation to solve
    let o_to_c = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * o_to_c.dot(ray.direction);
    let c = o_to_c.dot(o_to_c) - radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);
    (discriminant > 0.0)
}
