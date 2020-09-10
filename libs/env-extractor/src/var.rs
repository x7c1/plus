use crate::Error;
use crate::Error::{NotPresent, ParseError};
use std::env;
use std::env::VarError;
use std::fmt::Debug;
use std::str::FromStr;

pub struct EnvVar {
    pub key: String,
}

type EnvResult<Y, X> = crate::Result<Y, <X as FromStr>::Err>;

pub fn env_var<A: Into<String>>(key: A) -> EnvVar {
    EnvVar { key: key.into() }
}

impl EnvVar {
    pub fn as_optional<A>(&self) -> crate::Result<Option<A>, <A as FromStr>::Err>
    where
        A: FromStr,
        <A as FromStr>::Err: Debug,
    {
        let if_not_found = || Ok(None);
        let reify = |item| Ok(Some(item));
        self.parse(if_not_found, reify)
    }

    pub fn as_required<A>(&self) -> crate::Result<A, <A as FromStr>::Err>
    where
        A: FromStr,
        <A as FromStr>::Err: Debug,
    {
        let if_not_found = || Err(NotPresent(self.key.to_string()));
        let reify = |item| Ok(item);
        self.parse(if_not_found, reify)
    }

    fn parse<X, Y, F1, F2>(&self, if_not_found: F1, reify: F2) -> EnvResult<Y, X>
    where
        X: FromStr,
        <X as FromStr>::Err: Debug,
        F1: Fn() -> EnvResult<Y, X>,
        F2: Fn(X) -> EnvResult<Y, X>,
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

fn to_parse_error<E: Debug>(key: String, value: String, cause: E) -> Error<E> {
    ParseError { key, value, cause }
}
