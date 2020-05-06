use core::ops::Add;
use core::ops::Mul;

pub type Num = f64;

#[derive(PartialEq, Debug)]
pub struct Vec3 {
    x: Num,
    y: Num,
    z: Num,
}

impl Vec3 {
    pub fn new(x: Num, y: Num, z: Num) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn magnitude_squared(self) -> Num {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    pub fn magnitude(self) -> Num {
        Num::sqrt(self.magnitude_squared())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Mul<Num> for Vec3 {
    type Output = Self;

    fn mul(self, num: Num) -> Self {
        Self {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
        }
    }
}
