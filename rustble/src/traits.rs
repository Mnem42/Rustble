pub trait Player{
    fn lose(&mut self, bet: i64);
    fn win(&mut self, bet: i64);
    fn has_lost(self) -> bool;
    fn get_balance(&self) -> i64;
}

pub trait Randomiser{
    fn new() -> Self;
    fn random_bool(&mut self) -> bool;
    fn random_range(&mut self, min: u8, max: u8) -> u8;
}