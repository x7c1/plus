mod errors;
pub use errors::Error;
pub use errors::Result as ExtractorResult;

mod multiple;
pub use multiple::FromMultiple;

mod single;
pub use single::FromSingle;

use crate::errors::Error::ParseError;
use clap::ArgMatches;
use std::fmt::Debug;

pub struct Extractor<'a> {
    pub matches: &'a ArgMatches<'a>,
}

impl<'a> Extractor<'a> {
    pub fn new(matches: &'a ArgMatches) -> Extractor<'a> {
        Extractor { matches }
    }

    pub fn single<'k>(&self, key: &'k str) -> FromSingle<'k, 'a> {
        FromSingle {
            key,
            matches: self.matches,
        }
    }

    pub fn multiple<'k>(&self, key: &'k str) -> FromMultiple<'k, 'a> {
        FromMultiple {
            key,
            matches: self.matches,
        }
    }
}

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
