use reqwest::blocking::Body;
use sabi_core::auth::v4::canonical::HashedPayload;

pub struct RequestResource {
    pub body: Option<Body>,
    pub hash: HashedPayload,
}
