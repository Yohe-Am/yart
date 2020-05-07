use crate::math::*;
use rand::Rng;
use std::rc::Rc;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point, horizontal: Vec3, vertical: Vec3) -> Camera {
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::unit_z(); // FIXME: negative-z
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn standard() -> Camera {
        Camera {
            origin: Point::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            lower_left_corner: Point::new(-2.0, -1.0, -1.0),
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

                let record = HitRecord::new(position, solution, ray, outward_normal);
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
}

impl HitRecord {
    pub fn new(position: Point, t: Num, ray: &Ray, outward_normal: Vec3) -> HitRecord {
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

pub fn random_num_generator() -> impl FnMut() -> Num {
    let mut rng = rand::thread_rng();
    move || rng.gen()
}

pub fn random_num_generator_rng(min: Num, max: Num) -> impl FnMut() -> Num {
    let mut rng = rand::thread_rng();
    move || rng.gen_range(min, max)
}

pub fn random_vec3_generator() -> impl FnMut() -> Vec3 {
    let mut rng = rand::thread_rng();
    move || Vec3 {
        x: rng.gen(),
        y: rng.gen(),
        z: rng.gen(),
    }
}

pub fn random_vec3_generator_rng(min: Num, max: Num) -> impl FnMut() -> Vec3 {
    let mut rng = rand::thread_rng();
    move || Vec3 {
        x: rng.gen_range(min, max),
        y: rng.gen_range(min, max),
        z: rng.gen_range(min, max),
    }
}

/// Basic math types
#[allow(dead_code)]
pub mod math {
    use core::ops::{Add, Div, Mul, Neg, Sub};

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

        pub fn magnitude_squared(&self) -> Num {
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
        }
        pub fn magnitude(&self) -> Num {
            Num::sqrt(self.magnitude_squared())
        }

        pub fn unit_vector(&self) -> Vec3 {
            *self / self.magnitude()
        }

        pub fn dot(&self, other: Vec3) -> Num {
            (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
        }
        pub fn cross(&self, other: Vec3) -> Vec3 {
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
