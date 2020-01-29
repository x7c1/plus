use reqwest::blocking::Body;
use reqwest::header::HeaderMap;
use url::Url;

#[derive(Debug)]
pub struct InternalRequest {
    pub url: Url,
    pub body: Option<Body>,
    pub headers: HeaderMap,
}
