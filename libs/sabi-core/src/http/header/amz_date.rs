use crate::http::HeaderFragment;
use crate::http::ToHeaderFragment;
use crate::SabiResult;

use http::header::HeaderName;
use std::str::FromStr;

pub struct AmzDate(String);

impl AmzDate {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToHeaderFragment for AmzDate {
    fn into(self) -> SabiResult<HeaderFragment> {
        Ok(HeaderFragment {
            key: HeaderName::from_str("X-Amz-Date")?,
            value: self.as_str().parse()?,
        })
    }
}
