use crate::http::HeaderFragment;
use crate::http::ToHeaderFragment;
use crate::SabiResult;
use http::header::HeaderName;
use std::str::FromStr;

pub struct ContentLength(u64);

impl ContentLength {
    pub fn new(key: u64) -> Self {
        Self(key)
    }
}

impl ToHeaderFragment for ContentLength {
    fn into(self) -> SabiResult<HeaderFragment> {
        Ok(HeaderFragment {
            key: HeaderName::from_str("Content-Length")?,
            value: self.0.into(),
        })
    }
}
