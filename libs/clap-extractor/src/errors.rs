use std::fmt::Debug;

#[allow(unused)]
pub type Result<T, E> = ::std::result::Result<T, Error<E>>;

#[derive(Debug)]
pub enum Error<E>
where
    E: Debug,
{
    ParseError {
        key: String,
        value: String,
        cause: E,
    },
    NotFound(String),
}
