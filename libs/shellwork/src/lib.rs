#[macro_use]
extern crate failure;

pub mod core;

mod error;
pub use error::Error;
pub use error::Result;
