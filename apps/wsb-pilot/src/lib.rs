#[macro_use]
extern crate failure;

pub mod cmd;

mod error;
pub use error::Result as PilotResult;

mod mutable_self;
pub use mutable_self::MutableSelf;
