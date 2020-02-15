use chrono::{DateTime, Utc};
use reqwest::{Method, Url};
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::DateStamp;
use sabi_core::auth::v4::request::{AuthorizationRequest, CanonicalFragment};
use sabi_core::auth::v4::sign::CredentialScope;
use sabi_core::index::{RegionCode, ServiceCode};

pub struct RequestParts<'a> {
    pub url: Url,
    pub method: Method,
    pub requested_at: DateTime<Utc>,
    scope: CredentialScope<'a>,
    pub hashed_payload: HashedPayload,
}
impl<'a> RequestParts<'a> {
    pub fn new(
        url: Url,
        method: Method,
        region: &'a RegionCode,
        hashed_payload: HashedPayload,
        requested_at: DateTime<Utc>,
    ) -> RequestParts {
        let date = DateStamp::from(&requested_at);
        RequestParts {
            url,
            method,
            requested_at,
            scope: CredentialScope::v4(date, region, &ServiceCode::S3),
            hashed_payload,
        }
    }
}

impl AuthorizationRequest for RequestParts<'_> {
    fn to_canonical_fragment(&self) -> CanonicalFragment {
        CanonicalFragment {
            url: &self.url,
            method: &self.method,
            hashed_payload: &self.hashed_payload,
        }
    }

    fn to_scope(&self) -> &CredentialScope {
        &self.scope
    }

    fn to_datetime(&self) -> &DateTime<Utc> {
        &self.requested_at
    }
}
