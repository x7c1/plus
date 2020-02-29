use crate::internal;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct S3ErrorResponse {
    status: StatusCode,
    headers: HeaderMap,
    text: String,
}

impl S3ErrorResponse {
    pub async fn dump(response: reqwest::Response) -> internal::Result<S3ErrorResponse> {
        Ok(S3ErrorResponse {
            status: response.status(),
            headers: response.headers().clone(),
            text: response.text().await?,
        })
    }
}

impl fmt::Display for S3ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
