use crate::core;
use chrono::{DateTime, Utc};
use plus_aws::auth::v4::canonical::HashedPayload;
use plus_aws::http::request::header::ContentType;
use plus_aws::index::RegionCode;
use reqwest::Body;

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
