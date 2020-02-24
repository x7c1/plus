use crate::core;
use reqwest::header::HeaderMap;

mod error;
pub use error::Error;

mod e_tag;
pub use e_tag::ETag;

pub trait S3Header<'a>: Sized {
    const HEADER_NAME: &'a str;
    fn new<A: Into<String>>(a: A) -> core::Result<Self>;
}

pub trait S3HeaderMap {
    fn get_header_value<'a, A: S3Header<'a>>(&self) -> core::Result<A>;
}

impl S3HeaderMap for HeaderMap {
    fn get_header_value<'a, A: S3Header<'a>>(&self) -> core::Result<A> {
        let value = self
            .get(A::HEADER_NAME)
            .ok_or_else(|| Error::HeaderNotFound(A::HEADER_NAME.into()))?
            .to_str()
            .or_else(|cause| {
                Err(Error::InvalidCharacters {
                    name: A::HEADER_NAME.into(),
                    cause,
                })
            })
            .map(|value| A::new(value))?;

        Ok(value?)
    }
}
