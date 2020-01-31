#[macro_use]
extern crate failure;

pub mod auth;

mod error;
pub use error::Error;
pub use error::Result as SabiResult;

mod http;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
