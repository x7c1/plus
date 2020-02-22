#[macro_use]
extern crate failure;

#[macro_use]
pub mod operations;

mod internal;

pub mod core;

mod error;
pub use error::Error;
pub use error::Result;
