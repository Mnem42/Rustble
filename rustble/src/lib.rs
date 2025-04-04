use std::fmt::Debug;
use rand::{self, Rng};


#[derive(Debug)]
pub enum Error{
    OwningError
}

pub trait Player{
    fn new() -> Self;
    fn lose(&mut self) -> &mut Self;
    fn win(&mut self) -> &mut Self;
    fn has_lost(self) -> bool;
}

pub trait Randomiser{
    fn new() -> Self;
    fn random_bool(&mut self) -> bool;
    fn random_range(&mut self, min: u8, max: u8) -> u8;
}



#[derive(Debug,Copy,Clone)]
pub struct SampleRandom{
    iteration: u8
}

#[derive(Debug,Copy,Clone)]
struct SimplePlayer{
    has_lost: bool
}

impl Player for SimplePlayer{
    fn new() -> Self{
        SimplePlayer { has_lost: false }
    }

    fn lose(&mut self) -> &mut Self{
        self.has_lost=true;
        self
    }
    
    fn win(&mut self) -> &mut Self{
        self
    }

    fn has_lost(self) -> bool {
        self.has_lost
    }
}

impl Randomiser for SampleRandom{
    fn new() -> Self{
        SampleRandom { iteration: 0 }
    }

    fn random_bool(&mut self) -> bool {
        self.iteration+=1;

        println!("A: {:?}",self.iteration);
        (self.iteration % 6) == 0
    }

    fn random_range(&mut self, min: u8, max: u8) -> u8 {
        rand::rng().random_range(..=max)
    }
}
pub mod games;

