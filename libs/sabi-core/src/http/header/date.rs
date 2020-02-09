use crate::http::{HeaderFragment, ToHeaderFragment};
use crate::SabiResult;
use chrono::{DateTime, Utc};
use http::header::DATE;

pub struct Date(String);

impl Date {
    pub fn from(date: &DateTime<Utc>) -> Self {
        let value = date.to_rfc2822();
        Self::new(value)
    }

    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToHeaderFragment for Date {
    fn into(self) -> SabiResult<HeaderFragment> {
        Ok(HeaderFragment {
            key: DATE,
            value: self.as_str().parse()?,
        })
    }
}
