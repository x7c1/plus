use crate::to_parse_error;
use crate::CanExtractOptional;
use crate::CanExtractRequired;
use crate::Error;
use crate::Error::NotPresent;
use crate::ExtractorResult;

use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;

pub struct SingleValue {
    pub key: String,
}

type ResultFromStr<Y, X> = ExtractorResult<Y, <X as FromStr>::Err>;

impl SingleValue {
    pub fn new<A: Into<String>>(key: A) -> SingleValue {
        SingleValue { key: key.into() }
    }

    pub fn as_optional<A>(&self) -> Result<A, <Self as CanExtractOptional<A>>::Err>
    where
        Self: CanExtractOptional<A>,
    {
        self.get_optional()
    }

    pub fn as_required<A>(&self) -> Result<A, <Self as CanExtractRequired<A>>::Err>
    where
        Self: CanExtractRequired<A>,
    {
        self.get_required()
    }

    fn parse<X, Y, F1, F2>(&self, if_not_found: F1, reify: F2) -> ResultFromStr<Y, X>
    where
        X: FromStr,
        <X as FromStr>::Err: Debug,
        F1: Fn() -> ResultFromStr<Y, X>,
        F2: Fn(X) -> ResultFromStr<Y, X>,
    {
        let to_parsed = |value: String| {
            X::from_str(&value)
                .map_err(|e| to_parse_error(self.key.to_string(), value, e))
                .and_then(reify)
        };
        let maybe = match env::var(&self.key) {
            Ok(x1) => Some(x1),
            Err(VarError::NotPresent) => None,
            Err(e) => return Err(Error::EnvVarError(e)),
        };
        maybe.map(to_parsed).unwrap_or_else(if_not_found)
    }
}

impl<A: FromStr> CanExtractOptional<Option<A>> for SingleValue
where
    <A as FromStr>::Err: Debug,
{
    type Err = Error<<A as FromStr>::Err>;

    fn get_optional(&self) -> Result<Option<A>, Self::Err> {
        let if_not_found = || Ok(None);
        let reify = |item| Ok(Some(item));
        self.parse(if_not_found, reify)
    }
}

impl<A: FromStr> CanExtractRequired<A> for SingleValue
where
    <A as FromStr>::Err: Debug,
{
    type Err = Error<<A as FromStr>::Err>;

    fn get_required(&self) -> Result<A, Self::Err> {
        let if_not_found = || Err(NotPresent(self.key.to_string()));
        let reify = |item| Ok(item);
        self.parse(if_not_found, reify)
    }
}
