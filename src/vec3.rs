use crate::{
    helpers::{clamp, random_in},
    v3,
};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::ops;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Self) -> Self {
        Self {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn unit(&self) -> Self {
        *self / Vec3::from(self.len())
    }

    pub fn write(&self, samples_per_pixel: i32) {
        let num = 256.0;
        let mut r = self.x / f64::from(samples_per_pixel);
        let mut g = self.y / f64::from(samples_per_pixel);
        let mut b = self.z / f64::from(samples_per_pixel);

        r = r.sqrt();
        g = g.sqrt();
        b = b.sqrt();

        println!(
            "{} {} {}",
            (num * clamp(r, 0.0, 0.999)) as i32,
            (num * clamp(g, 0.0, 0.999)) as i32,
            (num * clamp(b, 0.0, 0.999)) as i32
        );
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = random_in::<Self>(v3!(-1), v3!(1));
            if p.len_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl From<f64> for Vec3 {
    fn from(num: f64) -> Self {
        Self {
            x: num,
            y: num,
            z: num,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * v3!(rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        v3!(self) * rhs
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / v3!(rhs)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        v3!(self) / rhs
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

pub type Color = Vec3;
pub type Point = Vec3;

#[macro_export]
macro_rules! v3 {
    ($f: expr) => {
        $crate::Vec3::from(f64::from($f))
    };
    ($x: expr, $y: expr, $z: expr) => {
        $crate::Vec3::new(f64::from($x), f64::from($y), f64::from($z))
    };
}

#[macro_export]
macro_rules! color {
    ($($e: expr),*) => {
        v3!($($e),*)
    };
}

#[macro_export]
macro_rules! point {
    ($($e: expr),*) => {
        v3!($($e),*)
    };
}
