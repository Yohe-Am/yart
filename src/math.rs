use core::ops::{Add, Div, Mul, Neg, Sub};
use rand::Rng;
use std::f64;

pub type Num = f64;

pub const EPSILON: f64 = f64::EPSILON;
pub const PI: f64 = f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;
pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;

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
use rand::rngs::ThreadRng;

pub fn get_rand_generator() -> rand::rngs::ThreadRng {
    static mut OPTION: Option<ThreadRng> = None;
    // option
    unsafe {
        match OPTION {
            Some(rng) => rng,
            None => {
                let rng = rand::thread_rng();
                OPTION = Some(rng);
                rng
            }
        }
    }
}
pub fn random_num_generator() -> impl FnMut() -> Num {
    let mut rng = rand::thread_rng();
    move || rng.gen()
}

pub fn random_num_generator_rng() -> impl FnMut(Num, Num) -> Num {
    let mut rng = rand::thread_rng();
    move |min, max| rng.gen_range(min, max)
}

pub fn random_num() -> Num {
    get_rand_generator().gen()
}

pub fn random_num_rng(min: Num, max: Num) -> Num {
    get_rand_generator().gen_range(min, max)
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

    pub fn random_vec3_generator_range() -> impl FnMut(Num, Num) -> Vec3 {
        let mut rng = rand::thread_rng();
        move |min, max| Vec3 {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_vec3() -> Vec3 {
        Vec3 {
            x: get_rand_generator().gen(),
            y: get_rand_generator().gen(),
            z: get_rand_generator().gen(),
        }
    }

    pub fn random_vec3_rng(min: Num, max: Num) -> Vec3 {
        Vec3 {
            x: get_rand_generator().gen_range(min, max),
            y: get_rand_generator().gen_range(min, max),
            z: get_rand_generator().gen_range(min, max),
        }
    }
}
#[cfg(test)]
mod test_vector3 {
    use super::vec3::*;

    #[test]
    fn test_magnitude() {
        let unit_vector = Vec3::one().unit_vector();
        assert_eq!(unit_vector.magnitude(), 1.00);
    }

    #[test]
    fn test_add_sub() {
        let sum = Vec3::one() + Vec3::one();
        let difference = Vec3::new(2.0, 2.0, 2.0) - sum;
        assert!(difference.magnitude().abs() <= super::EPSILON);
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
