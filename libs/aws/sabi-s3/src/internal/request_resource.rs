use reqwest::blocking::Body;
use sabi_core::auth::v4::canonical::PayloadHash;

pub struct RequestResource {
    pub body: Option<Body>,
    pub hash: PayloadHash,
}
