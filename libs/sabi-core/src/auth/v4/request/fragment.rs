use crate::auth::v4::canonical::HashedPayload;
use crate::auth::v4::request::Authorization;
use crate::http::Headers;
use crate::SabiResult;
use http::{HeaderMap, HeaderValue, Method};
use url::Url;

pub struct AuthorizationFragment {
    pub url: Url,
    pub method: Method,
    pub hashed_payload: HashedPayload,
}

impl AuthorizationFragment {
    pub fn to_header_value(&self, headers: &HeaderMap) -> SabiResult<HeaderValue> {
        unimplemented!()
    }
}
