use std::fmt::Debug;
use rand::{self, Rng};

pub mod traits;
pub mod randomisers;
pub mod games;

#[derive(Debug)]
pub enum Error{
    OwningError
}





#[derive(Debug,Copy,Clone)]
pub struct SampleRandom{
    iteration: u8
}

#[derive(Debug,Copy,Clone)]
struct SimplePlayer{
    has_lost: bool
}

