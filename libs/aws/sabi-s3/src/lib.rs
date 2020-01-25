#[macro_use]
extern crate failure;

#[macro_use]
pub mod operations;

mod client;
pub use client::S3Client;

mod errors;
pub use errors::Result as S3Result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
