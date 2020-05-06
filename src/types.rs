use core::ops::{Add, Div, Mul, Sub};
use std::f64::EPSILON;

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
            let sol = (-half_b - Num::sqrt(discriminant)) / a;
            if sol < t_max && sol > t_min {
                let position = ray.at(sol);
                let record = HitRecord {
                    position,
                    normal: (position - self.center) / self.radius, // unit vector
                    t: sol,
                };
                return Some(record);
            }
        }
        // didn't hit sphere
        None
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: Num, t_max: Num) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub position: Point,
    pub normal: Vec3,
    pub t: Num,
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: Num,
    pub y: Num,
    pub z: Num,
}

#[allow(dead_code)]
impl Vec3 {
    const EPSILON: Vec3 = Vec3 {
        x: EPSILON,
        y: EPSILON,
        z: EPSILON,
    };

    pub fn new<T: Into<Num>>(x: T, y: T, z: T) -> Vec3 {
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

pub type Num = f64;

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
