use crate::internal;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct ErrorResponse {
    status: StatusCode,
    headers: HeaderMap,
    text: String,
}

impl ErrorResponse {
    pub async fn dump(response: reqwest::Response) -> internal::Result<ErrorResponse> {
        Ok(ErrorResponse {
            status: response.status(),
            headers: response.headers().clone(),
            text: response.text().await?,
        })
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
