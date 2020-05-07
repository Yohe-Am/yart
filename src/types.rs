use crate::materials::*;
use crate::math::vec3::*;
use crate::math::*;
use std::rc::Rc;

pub mod materials {
    use super::*;

    pub trait Material {
        fn scatter(&self, ray_in: Ray, record: HitRecord) -> Option<(Ray, Color)>;
    }

    pub struct Dielectric {
        pub refraction_index: Num,
    }

    impl Material for Dielectric {
        fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<(Ray, Color)> {
            let etai_over_etat = if record.front_face {
                1.0 / self.refraction_index
            } else {
                self.refraction_index
            };

            let unit_direction = r_in.direction.unit_vector();

            let cos_theta = Num::min(-unit_direction.dot(record.normal), 1.0);
            let sin_theta = Num::sqrt(1.0 - cos_theta * cos_theta);

            let next_direction = if etai_over_etat * sin_theta > 1.0 {
                reflect(unit_direction, record.normal)
            } else {
                let reflect_prob = schlick(cos_theta, etai_over_etat);
                if rand::random::<f64>() < reflect_prob {
                    reflect(unit_direction, record.normal)
                } else {
                    refract(unit_direction, record.normal, etai_over_etat)
                }
            };
            Some((
                Ray {
                    origin: record.position,
                    direction: next_direction,
                },
                Color::one(),
            ))
        }
    }
    fn schlick(cosine: Num, ref_idx: Num) -> Num {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * Num::powi(1.0 - cosine, 5);
    }

    fn refract(uv: Vec3, normal: Vec3, etai_over_etat: Num) -> Vec3 {
        let cos_theta = -uv.dot(normal);
        let r_out_parallel = (uv + normal * cos_theta) * etai_over_etat;
        let r_out_perp = normal * -Num::sqrt(1.0 - r_out_parallel.magnitude_squared());
        r_out_parallel + r_out_perp
    }

    pub struct Metal {
        pub albedo: Color,
        fuzz: Num,
    }

    impl Metal {
        pub fn fuzz(&self) -> Num {
            self.fuzz
        }
        pub fn new(albedo: Color, fuzz: Num) -> Metal {
            Metal {
                albedo,
                fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
            }
        }
    }

    impl Material for Metal {
        fn scatter(&self, r_in: Ray, record: HitRecord) -> Option<(Ray, Color)> {
            let reflected = reflect(r_in.direction.unit_vector(), record.normal);
            if reflected.dot(record.normal) > 0.0 {
                Some((
                    Ray {
                        origin: record.position,
                        direction: reflected + (random_in_unit_sphere() * self.fuzz),
                    },
                    self.albedo,
                ))
            } else {
                None
            }
        }
    }
    fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
        let b = normal * vec.dot(normal);
        vec - (b * 2.0)
    }

    pub struct Lambertian {
        pub albedo: Color,
    }

    impl Material for Lambertian {
        fn scatter(&self, _: Ray, record: HitRecord) -> Option<(Ray, Color)> {
            let scatter_direction = record.normal + random_unit_vector();
            Some((
                Ray {
                    origin: record.position,
                    direction: scatter_direction,
                },
                self.albedo,
            ))
        }
    }

    // for lambertian diffuse
    fn random_unit_vector() -> Vec3 {
        let mut gen = random_num_generator_rng();
        let a = gen(0.0, 2.0 * math::PI);
        let z = gen(-1.0, 1.0);
        let r = Num::sqrt(1.0 - z * z);
        return Vec3::new(r * Num::cos(a), r * Num::sin(a), z);
    }

    fn random_in_unit_sphere() -> Vec3 {
        let mut gen = random_vec3_generator_rng();
        loop {
            let vec = gen(-1.0, 1.0);
            if vec.magnitude_squared() < 1.0 {
                return vec;
            }
        }
    }

    fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}
pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    // use_ctor_please: (),
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        aspect_ratio: Num,
        vertical_fov: Num,
    ) -> Camera {
        let theta = degrees_to_radians(vertical_fov);
        let half_height = Num::tan(theta / 2.0);
        let half_width = aspect_ratio * half_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner = lookfrom - (u * half_width) - (v * half_height) - w;

        let horizontal = u * half_width * 2.0;
        let vertical = v * half_height * 2.0;
        Camera {
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: Num, v: Num) -> Ray {
        Ray {
            origin: self.origin,
            direction: (self.lower_left_corner + (self.horizontal * u) + (self.vertical * v))
                - self.origin,
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: Num,
    pub material: Rc<dyn Material>,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        // (t^2 * b^2) + (2tb * (A−C)) + ((A−C) * (A−C)) − r^2 = 0
        // A = origin
        // b = direction
        // t = step
        // C = sphere center

        // use quadratic equation to solve
        // +- b * sqrt(b^2 * 4*a*c) / 2 * a

        // a = b^2  --  b dot b = |b|^2
        let a = ray.direction.magnitude_squared();

        let o_to_c = ray.origin - self.center; // (A - C)

        // b = 2b * (A - C) -- remove the 2
        let half_b = ray.direction.dot(o_to_c);

        // c = (A-C)^2 - r^2 -- again v dot b = |v|^2
        let c = o_to_c.magnitude_squared() - self.radius * self.radius;

        // b^2 * 4*a*c = (2*half_b)^2 - 4ac = 4halfb^2 - 4ac
        // = halfb^2 -ac (take common 4 out of root)
        let discriminant = (half_b * half_b) - (a * c);
        if discriminant > 0.0 {
            // hit sphere
            let root = Num::sqrt(discriminant);
            let mut solution = (-half_b - root) / a;
            let mut valid: bool = solution < t_max && solution > t_min;
            if !valid {
                solution = (-half_b - root) / a;
                valid = solution < t_max && solution > t_min;
            }
            if valid {
                let position = ray.at(solution);
                let outward_normal = (position - self.center) / self.radius;

                let record = HitRecord::new(
                    position,
                    solution,
                    ray,
                    outward_normal,
                    self.material.clone(),
                );
                return Some(record);
            }
        }
        // didn't hit sphere
        None
    }
}

pub type HittablesList = Vec<Rc<dyn Hit>>;

impl Hit for HittablesList {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord> {
        let mut record = None;
        let mut closest_so_far = t_max;

        for object in self {
            let temp = object.hit(&ray, t_min, closest_so_far);
            if let Some(r) = temp {
                closest_so_far = r.t;
                record = Some(r);
            }
        }
        record
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub position: Point,
    pub normal: Vec3,
    pub t: Num,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        position: Point,
        t: Num,
        ray: &Ray,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            position,
            t,
            front_face,
            normal,
            material,
        }
    }

    pub fn set_normal(self, ray: &Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            position: self.position,
            t: self.t,
            front_face,
            normal,
            material: self.material,
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: Num) -> Point {
        self.origin + (self.direction * t)
    }
}

pub type Point = Vec3;

pub type Color = Vec3;

impl Color {
    pub fn ppm_fmt(self) -> String {
        format!(
            "{} {} {}\n",
            (255.999 * self.x) as i32,
            (255.999 * self.y) as i32,
            (255.999 * self.z) as i32
        )
    }
}

/// Basic math types
#[allow(dead_code)]
pub mod math {
    use core::ops::{Add, Div, Mul, Neg, Sub};
    use rand::Rng;

    pub type Num = f64;

    pub const EPSILON: f64 = std::f64::EPSILON;
    pub const PI: f64 = std::f64::consts::PI;
    pub const INFINITY: f64 = std::f64::INFINITY;
    pub const NEG_INFINITY: f64 = std::f64::NEG_INFINITY;

    pub fn clamp_num(value: Num, min: Num, max: Num) -> Num {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    pub fn degrees_to_radians(degrees: Num) -> Num {
        degrees * PI / 180.0
    }

    pub fn random_num_generator() -> impl FnMut() -> Num {
        let mut rng = rand::thread_rng();
        move || rng.gen()
    }

    pub fn random_num_generator_rng() -> impl FnMut(Num, Num) -> Num {
        let mut rng = rand::thread_rng();
        move |min, max| rng.gen_range(min, max)
    }
    pub mod vec3 {
        use super::*;

        #[derive(PartialEq, Debug, Clone, Copy)]
        pub struct Vec3 {
            pub x: Num,
            pub y: Num,
            pub z: Num,
        }

        impl Vec3 {
            pub const EPSILON_VEC3: Vec3 = Vec3 {
                x: EPSILON,
                y: EPSILON,
                z: EPSILON,
            };

            pub fn new<T, U, V>(x: T, y: U, z: V) -> Vec3
            where
                T: Into<Num>,
                U: Into<Num>,
                V: Into<Num>,
            {
                Vec3 {
                    x: x.into(),
                    y: y.into(),
                    z: z.into(),
                }
            }

            pub fn zero() -> Vec3 {
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                }
            }

            pub fn one() -> Vec3 {
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }
            }

            pub fn unit_x() -> Vec3 {
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }
            }

            pub fn unit_y() -> Vec3 {
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }
            }

            pub fn unit_z() -> Vec3 {
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                }
            }

            pub fn magnitude_squared(self) -> Num {
                (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
            }
            pub fn magnitude(self) -> Num {
                Num::sqrt(self.magnitude_squared())
            }

            pub fn unit_vector(self) -> Vec3 {
                self / self.magnitude()
            }

            pub fn dot(self, other: Vec3) -> Num {
                (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
            }
            pub fn cross(self, other: Vec3) -> Vec3 {
                Vec3 {
                    x: (self.y * other.z) - (self.z * other.y),
                    y: (self.z * other.x) - (self.x * other.z),
                    z: (self.x * other.y) - (self.y * other.x),
                }
            }
        }

        impl std::fmt::Display for Vec3 {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {} {}", self.x, self.y, self.z)
            }
        }

        impl Add for Vec3 {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Vec3 {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl Sub for Vec3 {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Vec3 {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }

        impl Mul for Vec3 {
            //TODO: replace this with something more explicit?
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Vec3 {
                    x: self.x * other.x,
                    y: self.y * other.y,
                    z: self.z * other.z,
                }
            }
        }

        impl Mul<Num> for Vec3 {
            type Output = Self;

            fn mul(self, num: Num) -> Self {
                Vec3 {
                    x: self.x * num,
                    y: self.y * num,
                    z: self.z * num,
                }
            }
        }

        impl Div<Num> for Vec3 {
            type Output = Self;

            fn div(self, num: Num) -> Self {
                self * (1.0 / num)
            }
        }

        impl Neg for Vec3 {
            type Output = Vec3;

            fn neg(self) -> Self::Output {
                Vec3 {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }

        pub fn random_vec3_generator() -> impl FnMut() -> Vec3 {
            let mut rng = rand::thread_rng();
            move || Vec3 {
                x: rng.gen(),
                y: rng.gen(),
                z: rng.gen(),
            }
        }

        pub fn random_vec3_generator_rng() -> impl FnMut(Num, Num) -> Vec3 {
            let mut rng = rand::thread_rng();
            move |min, max| Vec3 {
                x: rng.gen_range(min, max),
                y: rng.gen_range(min, max),
                z: rng.gen_range(min, max),
            }
        }
    }
}

#[cfg(test)]
mod test_ray {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::one(),
        };
        assert_eq!(ray.at(5.0), Vec3::new(5, 5, 5));
    }
}

#[cfg(test)]
mod test_vector3 {
    use super::*;

    #[test]
    fn test_magnitude() {
        let unit_vector = Vec3::one().unit_vector();
        assert_eq!(unit_vector.magnitude(), 1.00);
    }

    #[test]
    fn test_add_sub() {
        let sum = Vec3::one() + Vec3::one();
        let difference = Vec3::new(2.0, 2.0, 2.0) - sum;
        assert!(difference.magnitude().abs() <= EPSILON);
    }

    #[test]
    fn test_mul_div() {
        assert_eq!(Vec3::new(1, 2, 3) * Vec3::new(3, 2, 1), Vec3::new(3, 4, 3));
        assert_eq!(Vec3::one() / 4.0, Vec3::new(0.25, 0.25, 0.25));
    }

    #[test]
    fn test_dot() {
        assert_eq!(Vec3::unit_x().dot(Vec3::unit_y()), 0.0);
    }

    #[test]
    fn test_cross() {
        assert_eq!(Vec3::unit_x().cross(Vec3::unit_y()), Vec3::unit_z());
    }
}
