#[macro_use]
extern crate failure;

extern crate derive_sabi;

#[macro_use]
pub mod operations;

mod client;
pub use client::S3Client;

mod errors;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
