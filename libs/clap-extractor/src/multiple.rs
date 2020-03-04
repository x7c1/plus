use crate::iter::FallibleResults;
use crate::{to_parse_error, CanExtractOptional, Error};
use clap::{ArgMatches, Values};
use std::fmt::Debug;
use std::str::FromStr;

pub struct FromMultiple<'k, 'v> {
    pub key: &'k str,
    pub matches: &'v ArgMatches<'v>,
}

impl<'k, 'v> FromMultiple<'k, 'v> {
    pub fn as_optional<A>(&self) -> Result<A, <Self as CanExtractOptional<A>>::Err>
    where
        FromMultiple<'k, 'v>: CanExtractOptional<A>,
    {
        self.get_optional()
    }
}

impl<A: FromStr> CanExtractOptional<Vec<A>> for FromMultiple<'_, '_>
where
    <A as FromStr>::Err: Debug,
{
    type Err = Error<<A as FromStr>::Err>;

    fn get_optional(&self) -> Result<Vec<A>, Self::Err> {
        let to_parsed = |value| {
            let reified = A::from_str(value);
            reified.map_err(|e| to_parse_error(self.key, value, e))
        };
        self.matches
            .values_of(self.key)
            .unwrap_or_else(Values::default)
            .map(to_parsed)
            .complete_or_failed()
    }
}
