use rand::random;

pub trait Random {
    fn random() -> Self;
    fn random_in(min: Self, max: Self) -> Self;
}

impl Random for f64 {
    fn random() -> Self {
        random()
    }

    fn random_in(min: Self, max: Self) -> Self {
        min + (max - min) * random::<Self>()
    }
}
