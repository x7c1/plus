#[macro_use]
extern crate failure;

mod error;
pub use error::Result as PilotResult;

mod mutable_self;
pub use mutable_self::MutableSelf;
