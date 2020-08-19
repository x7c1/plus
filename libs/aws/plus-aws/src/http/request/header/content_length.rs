use crate::http::request::HeaderFragment;
use crate::http::request::ToHeaderFragment;
use http::header::HeaderName;
use std::str::FromStr;

pub struct ContentLength(u64);

impl ContentLength {
    pub fn new(key: u64) -> Self {
        Self(key)
    }
}

impl ToHeaderFragment for ContentLength {
    fn into(self) -> crate::Result<HeaderFragment> {
        Ok(HeaderFragment {
            key: HeaderName::from_str("Content-Length")?,
            value: self.0.into(),
        })
    }
}
