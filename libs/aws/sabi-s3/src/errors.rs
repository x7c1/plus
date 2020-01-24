extern crate failure;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "std::io::Error > {}", 0)]
    StdIoError(std::io::Error),
}
