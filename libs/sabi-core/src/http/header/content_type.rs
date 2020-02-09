use crate::http::HeaderFragment;
use crate::SabiResult;
use http::header::{HeaderName, CONTENT_TYPE};
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

impl<'a> Into<HeaderFragment<HeaderName, String>> for ContentType {
    fn into(self) -> HeaderFragment<HeaderName, String> {
        HeaderFragment {
            key: CONTENT_TYPE,
            value: self.0,
        }
    }
}

impl<'a> Into<HeaderFragment<HeaderName, String>> for &ContentType {
    fn into(self) -> HeaderFragment<HeaderName, String> {
        HeaderFragment {
            key: CONTENT_TYPE,
            value: self.0.to_string(),
        }
    }
}

impl FromStr for ContentType {
    type Err = crate::Error;

    fn from_str(s: &str) -> SabiResult<Self> {
        Ok(Self::new(s))
    }
}
