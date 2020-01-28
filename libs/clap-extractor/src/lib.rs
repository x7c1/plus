mod errors;
pub use errors::Error;
pub use errors::Result as ExtractorResult;

mod iter;

mod matcher;
pub use matcher::Matcher;

mod multiple;
pub use multiple::FromMultiple;

mod single;
pub use single::FromSingle;

use crate::errors::Error::ParseError;
use std::fmt::Debug;

pub trait CanExtractOptional<A> {
    type Err;
    fn get_optional(&self) -> Result<A, Self::Err>;
}

pub trait CanExtractRequired<A> {
    type Err;
    fn get_required(&self) -> Result<A, Self::Err>;
}

fn to_parse_error<E: Debug>(key: &str, value: &str, cause: E) -> Error<E> {
    ParseError {
        key: key.to_string(),
        value: value.to_string(),
        cause,
    }
}
