pub trait Player{
    //fn new() -> Self;
    fn lose(&mut self, bet: i64);
    fn win(&mut self, bet: i64);
    fn has_lost(self) -> bool;
}

pub trait Randomiser{
    fn new() -> Self;
    fn random_bool(&mut self) -> bool;
    fn random_range(&mut self, min: u8, max: u8) -> u8;
}