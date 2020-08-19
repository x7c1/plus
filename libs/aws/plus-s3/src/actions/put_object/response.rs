use crate::core;
use crate::core::response::headers::{AwsHeaderMap, ETag};
use reqwest::header::HeaderMap;

#[derive(Debug)]
pub struct Response {
    pub headers: ResponseHeaders,
}

#[derive(Debug)]
pub struct ResponseHeaders {
    pub e_tag: ETag,
}

impl ResponseHeaders {
    pub fn from(map: &HeaderMap) -> core::Result<ResponseHeaders> {
        Ok(ResponseHeaders {
            e_tag: map.as_required()?,
        })
    }
}
