use std::env::VarError;
use std::fmt::Debug;

#[allow(unused)]
pub type Result<T, E> = ::std::result::Result<T, Error<E>>;

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
