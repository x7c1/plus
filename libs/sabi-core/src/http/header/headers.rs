use crate::http::HeaderFragment;
use crate::SabiResult;
use http::header::{IntoHeaderName, InvalidHeaderValue};
use http::{HeaderMap, HeaderValue};
use std::convert::TryInto;

pub trait Headers: Sized {
    fn push<A, K, V>(self, header: A) -> SabiResult<Self>
    where
        A: Into<HeaderFragment<K, V>>,
        K: IntoHeaderName,
        V: TryInto<HeaderValue, Error = InvalidHeaderValue>;
}

impl Headers for HeaderMap {
    fn push<A, K, V>(mut self, header: A) -> SabiResult<Self>
    where
        A: Into<HeaderFragment<K, V>>,
        K: IntoHeaderName,
        V: TryInto<HeaderValue, Error = InvalidHeaderValue>,
    {
        let fragment = header.into();
        let value = fragment.value.try_into()?;
        self.insert(fragment.key, value);
        Ok(self)
    }
}
