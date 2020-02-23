extern crate failure;

use crate::actions;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "actions::get_object::Error > {}", 0)]
    GetObjectError(actions::get_object::Error),

    #[fail(display = "actions::put_object::Error > {}", 0)]
    PutObjectError(actions::put_object::Error),
}

impl From<actions::get_object::Error> for Error {
    fn from(e: actions::get_object::Error) -> Self {
        Error::GetObjectError(e)
    }
}

impl From<actions::put_object::Error> for Error {
    fn from(e: actions::put_object::Error) -> Self {
        Error::PutObjectError(e)
    }
}
