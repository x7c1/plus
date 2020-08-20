use http::header::HeaderName;
use http::HeaderValue;

pub struct HeaderFragment {
    pub key: HeaderName,
    pub value: HeaderValue,
}

pub trait ToHeaderFragment {
    fn into(self) -> crate::Result<HeaderFragment>;
}

impl<'a> ToHeaderFragment for (&'static str, &'a str) {
    fn into(self) -> crate::Result<HeaderFragment> {
        let (key, value) = self;
        Ok(HeaderFragment {
            key: key.parse()?,
            value: value.parse()?,
        })
    }
}

impl ToHeaderFragment for (&'static str, u64) {
    fn into(self) -> crate::Result<HeaderFragment> {
        let (key, value) = self;
        Ok(HeaderFragment {
            key: key.parse()?,
            value: value.into(),
        })
    }
}
