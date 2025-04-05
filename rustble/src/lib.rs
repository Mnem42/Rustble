use std::fmt::Debug;

pub mod traits;
pub mod randomisers;
pub mod games;

#[derive(Debug)]
pub enum Error{
    OwningError
}