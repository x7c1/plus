use reqwest::header::HeaderMap;
use reqwest::Body;
use reqwest::Method;
use url::Url;

#[derive(Debug)]
pub struct InternalRequest {
    pub url: Url,
    pub method: Method,
    pub body: Option<Body>,
    pub headers: HeaderMap,
}
