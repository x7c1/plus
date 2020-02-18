mod error;
pub use error::Error;
pub use error::FromStrResult;
pub use error::Result as ExtractorResult;

mod single;
pub use single::SingleValue;

use crate::Error::ParseError;
use std::fmt::Debug;

pub trait CanExtractOptional<A> {
    type Err;
    fn get_optional(&self) -> Result<A, Self::Err>;
}

pub trait CanExtractRequired<A> {
    type Err;
    fn get_required(&self) -> Result<A, Self::Err>;
}

fn to_parse_error<E: Debug>(key: String, value: String, cause: E) -> Error<E> {
    ParseError { key, value, cause }
}
