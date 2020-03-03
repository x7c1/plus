use crate::core;
use chrono::{DateTime, Utc};
use reqwest::Body;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::http::header::ContentType;
use sabi_core::index::RegionCode;

pub struct RequestResource<'a> {
    pub body: Option<Body>,
    pub hash: HashedPayload,
    pub region: Option<&'a RegionCode>,
    pub content_type: Option<&'a ContentType>,
    pub content_length: u64,
    pub requested_at: DateTime<Utc>,
}

#[async_trait]
pub trait ResourceLoader {
    async fn load(&self) -> core::Result<RequestResource>;
}
