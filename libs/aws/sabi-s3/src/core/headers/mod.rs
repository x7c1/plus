use crate::core::{ETag, S3Result};
use reqwest::header::HeaderMap;

mod error;
pub use error::Error;

pub trait S3HeaderMap {
    fn get_e_tag(&self) -> S3Result<ETag>;
}

impl S3HeaderMap for HeaderMap {
    fn get_e_tag(&self) -> S3Result<ETag> {
        let e_tag = self
            .get("ETag")
            .ok_or_else(|| Error::ETagHeaderNotFound)?
            .to_str()
            .or_else(|e| Err(Error::InvalidETagHeader(e)))
            .map(|value| ETag::new(value))?;

        Ok(e_tag)
    }
}
