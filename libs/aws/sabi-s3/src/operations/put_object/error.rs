extern crate failure;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "FileNotFound > {}", 0)]
    FileNotFound { path: String, description: String },
}
