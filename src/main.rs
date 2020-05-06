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
    let t = hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let n = ray.at(t).unit_vector() - Vec3::new(0.0, 0.0, -0.1);
        // return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
        return (n + Vec3::one()) * 0.5;
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    (Color::one() * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t)
    //^ white                     ^ blue
}

fn hit_sphere(center: Point, radius: Num, ray: &Ray) -> Num {
    // (t^2 * b^2) + (2tb * (A−C)) + ((A−C) * (A−C)) − r^2 = 0
    // A = origin
    // b = direction
    // t = step
    // C = sphere center

    // use quadratic equation to solve
    // +- b * sqrt(b^2 * 4*a*c) / 2 * a

    // a = b^2  --  b dot b = |b|^2
    let a = ray.direction.magnitude_squared();

    let o_to_c = ray.origin - center; // (A - C)

    // b = 2b * (A - C) -- remove the 2
    let half_b = ray.direction.dot(o_to_c);

    // c = (A-C)^2 - r^2 -- again v dot b = |v|^2
    let c = o_to_c.magnitude_squared() - radius * radius;

    // b^2 * 4*a*c = (2*half_b)^2 - 4ac = 4halfb^2 - 4ac
    // = halfb^2 -ac (take common 4 out of root)
    let discriminant = (half_b * half_b) - (a * c);
    if discriminant > 0.0 {
        // hit sphere
        return (-half_b - Num::sqrt(discriminant)) / a;
    }
    // didn't hit sphere
    -1.0
}
