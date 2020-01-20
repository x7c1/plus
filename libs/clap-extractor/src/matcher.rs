use crate::{FromMultiple, FromSingle};
use clap::ArgMatches;

pub trait Matcher {
    fn single<'k, 'v>(&'v self, key: &'k str) -> FromSingle<'k, 'v>;
    fn multiple<'k, 'v>(&'v self, key: &'k str) -> FromMultiple<'k, 'v>;
}

impl Matcher for ArgMatches<'_> {
    fn single<'k, 'v>(&'v self, key: &'k str) -> FromSingle<'k, 'v> {
        FromSingle { key, matches: self }
    }
    fn multiple<'k, 'v>(&'v self, key: &'k str) -> FromMultiple<'k, 'v> {
        FromMultiple { key, matches: self }
    }
}
