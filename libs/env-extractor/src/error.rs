use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;

#[allow(unused)]
pub type Result<T, E> = ::std::result::Result<T, Error<E>>;

pub type FromStrResult<T> = Result<T, <String as FromStr>::Err>;

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
