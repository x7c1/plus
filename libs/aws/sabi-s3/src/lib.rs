#[macro_use]
extern crate failure;

#[macro_use]
pub mod operations;

mod internal;

pub mod core;

mod error;
pub use error::Result as S3Result;

pub mod verbs;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
