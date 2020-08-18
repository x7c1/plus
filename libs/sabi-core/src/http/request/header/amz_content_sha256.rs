use crate::http::request::HeaderFragment;
use crate::http::request::ToHeaderFragment;
use crate::PlusResult;
use http::header::HeaderName;
use std::str::FromStr;

pub struct AmzContentSha256(String);

impl AmzContentSha256 {
    pub fn new<A: Into<String>>(key: A) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl ToHeaderFragment for AmzContentSha256 {
    fn into(self) -> PlusResult<HeaderFragment> {
        Ok(HeaderFragment {
            key: HeaderName::from_str("X-Amz-Content-Sha256")?,
            value: self.as_str().parse()?,
        })
    }
}
