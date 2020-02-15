use crate::http::HeaderFragment;
use crate::http::ToHeaderFragment;
use crate::SabiResult;

use http::header::CONTENT_TYPE;
use std::str::FromStr;

#[derive(Debug)]
pub struct ContentType(String);

impl ContentType {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn application_octet_stream() -> ContentType {
        ContentType::new("application/octet-stream")
    }
}

impl ToHeaderFragment for ContentType {
    fn into(self) -> SabiResult<HeaderFragment> {
        Ok(HeaderFragment {
            key: CONTENT_TYPE,
            value: self.as_str().parse()?,
        })
    }
}

impl ToHeaderFragment for &ContentType {
    fn into(self) -> SabiResult<HeaderFragment> {
        Ok(HeaderFragment {
            key: CONTENT_TYPE,
            value: self.as_str().parse()?,
        })
    }
}

impl FromStr for ContentType {
    type Err = crate::Error;

    fn from_str(s: &str) -> SabiResult<Self> {
        Ok(Self::new(s))
    }
}
