use crate::http::HeaderFragment;
use http::header::HeaderName;

pub struct AmzDate(String);

impl AmzDate {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Into<HeaderFragment<HeaderName, String>> for AmzDate {
    fn into(self) -> HeaderFragment<HeaderName, String> {
        HeaderFragment {
            key: HeaderName::from_static("X-Amz-Date"),
            value: self.0,
        }
    }
}
