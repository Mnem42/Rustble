use rand::Rng;

use crate::traits::Randomiser;

#[derive(Clone)]
pub struct SimpleRandom;

impl Randomiser for SimpleRandom{
    fn new() -> Self {
        SimpleRandom {}
    }

    fn random_bool(&mut self) -> bool {
        rand::rng().random_bool(0.5)
    }

    fn random_range(&mut self, min: u8, max: u8) -> u8 {
        rand::rng().random_range(min..max)
    }
}