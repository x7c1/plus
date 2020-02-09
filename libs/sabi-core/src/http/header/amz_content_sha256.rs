use crate::http::HeaderFragment;
use http::header::HeaderName;

pub struct AmzContentSha256(String);

impl AmzContentSha256 {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Into<HeaderFragment<HeaderName, String>> for AmzContentSha256 {
    fn into(self) -> HeaderFragment<HeaderName, String> {
        HeaderFragment {
            key: HeaderName::from_static("X-Amz-Content-Sha256"),
            value: self.0,
        }
    }
}
