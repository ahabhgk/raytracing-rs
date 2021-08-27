use rand::{self, distributions::Standard, prelude::Distribution};
use std::ops;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random<T>() -> T
where
    Standard: Distribution<T>,
{
    rand::random()
}

pub fn random_in<T>(min: T, max: T) -> T
where
    Standard: Distribution<T>,
    T: ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Add<Output = T> + Copy,
{
    min + (max - min) * random::<T>()
}
