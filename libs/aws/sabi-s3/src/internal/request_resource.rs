use crate::internal;
use chrono::{DateTime, Utc};
use reqwest::blocking::Body;
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::http::header::ContentType;
use sabi_core::index::RegionCode;

pub struct RequestResource<'a> {
    pub body: Option<Body>,
    pub hash: HashedPayload,
    pub region: Option<&'a RegionCode>,
    pub content_type: Option<&'a ContentType>,
    pub requested_at: DateTime<Utc>,
}

pub trait ResourceLoader {
    fn load(&self) -> internal::Result<RequestResource>;
}
