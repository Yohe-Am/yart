use core::ops::{Add, Div, Mul, Sub};
use std::f64::EPSILON;

pub type Num = f64;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: Num,
    pub y: Num,
    pub z: Num,
}

#[allow(dead_code)]
impl Vector3 {
    const EPSILON: Vector3 = Vector3 {
        x: EPSILON,
        y: EPSILON,
        z: EPSILON,
    };

    pub fn new(x: Num, y: Num, z: Num) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Vector3 {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn unit_x() -> Vector3 {
        Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_y() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn unit_z() -> Vector3 {
        Vector3 {
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

    pub fn unit_vector(self) -> Vector3 {
        self / self.magnitude()
    }

    pub fn dot(self, other: Vector3) -> Num {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    pub fn cross(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }
}

impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for Vector3 {
    //TODO: replace this with something more explicit?
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Num> for Vector3 {
    type Output = Self;

    fn mul(self, num: Num) -> Self {
        Vector3 {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
        }
    }
}

impl Div<Num> for Vector3 {
    type Output = Self;

    fn div(self, num: Num) -> Self {
        self * (1.0 / num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let unit_vector = Vector3::one().unit_vector();
        assert_eq!(unit_vector.magnitude(), 1.00);
    }

    #[test]
    fn test_add_sub() {
        let sum = Vector3::one() + Vector3::one();
        let difference = Vector3::new(2.0, 2.0, 2.0) - sum;
        assert!(difference.magnitude().abs() <= EPSILON);
    }

    #[test]
    fn test_dot() {
        assert_eq!(Vector3::unit_x().dot(Vector3::unit_y()), 0.0);
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vector3::unit_x().cross(Vector3::unit_y()),
            Vector3::unit_z()
        );
    }
}
