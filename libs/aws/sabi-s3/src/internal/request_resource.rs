use crate::core::S3Result;
use reqwest::blocking::Body;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::http::header::ContentType;
use sabi_core::index::RegionCode;

pub struct RequestResource {
    pub body: Option<Body>,
    pub hash: HashedPayload,
    pub region: RegionCode,
    pub content_type: ContentType,
}

pub trait ResourceProvider {
    fn provide(self) -> S3Result<RequestResource>;
}
