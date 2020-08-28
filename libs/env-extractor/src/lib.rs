mod error;
pub use error::Error;
pub use error::FromStrResult;
pub use error::Result;

mod single;
pub use single::SingleValue;

use crate::Error::ParseError;
use std::fmt::Debug;

fn to_parse_error<E: Debug>(key: String, value: String, cause: E) -> Error<E> {
    ParseError { key, value, cause }
}
