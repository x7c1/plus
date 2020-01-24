use crate::{to_parse_error, CanExtractOptional, Error};
use clap::ArgMatches;
use std::fmt::Debug;
use std::str::FromStr;

pub struct FromMultiple<'k, 'v> {
    pub key: &'k str,
    pub matches: &'v ArgMatches<'v>,
}

impl<'k, 'v> FromMultiple<'k, 'v> {
    #[allow(dead_code)]
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
        let values = if let Some(values) = self.matches.values_of(self.key) {
            values
        } else {
            return Ok(vec![]);
        };
        let mut items = vec![];
        for value in values {
            match FromStr::from_str(value) {
                Ok(item) => items.push(item),
                Err(cause) => return Err(to_parse_error(self.key, value, cause)),
            }
        }
        Ok(items)
    }
}
