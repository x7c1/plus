#[macro_use]
extern crate failure;

mod client;
pub use client::S3Client;

mod errors;
pub mod operations;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
