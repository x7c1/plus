#[macro_use]
extern crate failure;

pub mod auth;
pub mod env;

mod error;
pub use error::Error;
pub use error::Result as SabiResult;

pub mod http;
pub mod index;
pub mod io;
pub mod verbs;
