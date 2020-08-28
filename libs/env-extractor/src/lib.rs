mod error;
pub use error::Error;
pub use error::Result;
pub type RequiredResult<A> = Result<A, <A as FromStr>::Err>;
pub type OptionalResult<A> = Result<Option<A>, <A as FromStr>::Err>;

mod single;
pub use single::EnvVar;

use crate::Error::ParseError;
use std::fmt::Debug;
use std::str::FromStr;

fn to_parse_error<E: Debug>(key: String, value: String, cause: E) -> Error<E> {
    ParseError { key, value, cause }
}
