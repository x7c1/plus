#[macro_use]
extern crate failure;

#[macro_use]
pub mod operations;

mod client;
mod internal;
pub use client::S3Client;

pub mod core;

mod errors;
pub use errors::Result as S3Result;

pub mod verbs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
