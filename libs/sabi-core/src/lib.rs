#[macro_use]
extern crate failure;

pub mod auth;

mod error;
pub use error::Error;
pub use error::Result as SabiResult;

pub mod http;

mod index;

pub mod verbs;
