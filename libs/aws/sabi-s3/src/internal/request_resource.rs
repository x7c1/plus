use crate::core::S3Result;
use chrono::{DateTime, Utc};
use reqwest::blocking::Body;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::http::header::ContentType;
use sabi_core::index::RegionCode;

pub struct RequestResource {
    pub body: Option<Body>,
    pub hash: HashedPayload,
    pub region: Option<RegionCode>,
    pub content_type: ContentType,
    pub requested_at: DateTime<Utc>,
}

pub trait ResourceProvider {
    fn provide(self) -> S3Result<RequestResource>;
}
