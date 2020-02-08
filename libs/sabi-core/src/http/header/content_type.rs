use crate::http::HeaderFragment;
use http::header::{HeaderName, CONTENT_TYPE};

#[derive(Debug)]
pub struct ContentType(String);

impl ContentType {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
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
