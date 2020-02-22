extern crate failure;

use crate::operations;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "operations::get_object::Error > {}", 0)]
    GetObjectError(operations::get_object::Error),

    #[fail(display = "operations::put_object::Error > {}", 0)]
    PutObjectError(operations::put_object::Error),

    #[fail(display = "operations::Error > {}", 0)]
    S3ClientError(operations::Error),
}

impl From<operations::get_object::Error> for Error {
    fn from(e: operations::get_object::Error) -> Self {
        Error::GetObjectError(e)
    }
}

impl From<operations::put_object::Error> for Error {
    fn from(e: operations::put_object::Error) -> Self {
        Error::PutObjectError(e)
    }
}

impl From<operations::Error> for Error {
    fn from(e: operations::Error) -> Self {
        Error::S3ClientError(e)
    }
}
