use std::ops;

use crate::{random::Random, v3};

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

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

    pub fn to_rgb_string(&self, samples_per_pixel: i32) -> String {
        let num = 256.0;
        let mut r = self.x / f64::from(samples_per_pixel);
        let mut g = self.y / f64::from(samples_per_pixel);
        let mut b = self.z / f64::from(samples_per_pixel);

        r = r.sqrt();
        g = g.sqrt();
        b = b.sqrt();

        format!(
            "{} {} {}",
            (num * clamp(r, 0.0, 0.999)) as i32,
            (num * clamp(g, 0.0, 0.999)) as i32,
            (num * clamp(b, 0.0, 0.999)) as i32
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in(v3!(-1), v3!(1));
            if p.len_squared() < 1.0 {
                return p;
            }
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

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = v3!(f64::random_in(-1.0, 1.0), f64::random_in(-1.0, 1.0), 0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(&self, v: &Self) -> Self {
        *self - 2.0 * self.dot(v) * *v
    }

    pub fn refract(&self, v: &Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = ((-*self).dot(v)).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *v);
        let r_out_parallel = -(1.0 - r_out_perp.len_squared()).abs().sqrt() * *v;
        r_out_perp + r_out_parallel
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

impl ops::Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        self + v3!(rhs)
    }
}

impl ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        v3!(self) + rhs
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

impl ops::Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self - v3!(rhs)
    }
}

impl ops::Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        v3!(self) - rhs
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

impl Random for Vec3 {
    fn random() -> Self {
        Self {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        }
    }

    fn random_in(min: Self, max: Self) -> Self {
        min + (max - min) * Self::random()
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
        $crate::v3!($($e),*)
    };
}

#[macro_export]
macro_rules! point {
    ($($e: expr),*) => {
        $crate::v3!($($e),*)
    };
}
