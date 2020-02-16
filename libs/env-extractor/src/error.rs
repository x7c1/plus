use std::convert;
use std::env::VarError;
use std::fmt::Debug;

#[allow(unused)]
pub type Result<T, E> = ::std::result::Result<T, Error<E>>;

pub type InfallibleResult<T> = Result<T, convert::Infallible>;

#[derive(Debug)]
pub enum Error<E>
where
    E: Debug,
{
    EnvVarError(VarError),
    NotPresent(String),
    ParseError {
        key: String,
        value: String,
        cause: E,
    },
}
