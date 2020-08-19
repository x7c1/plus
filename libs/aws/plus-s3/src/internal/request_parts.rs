use chrono::{DateTime, Utc};
use plus_aws::auth::v4::canonical::{HashedPayload, HeadersCapturer};
use plus_aws::auth::v4::chrono::DateStamp;
use plus_aws::auth::v4::request::AuthorizationParts;
use plus_aws::auth::v4::sign::CredentialScope;
use plus_aws::index::{RegionCode, ServiceCode};
use reqwest::{Method, Url};

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

impl AuthorizationParts for RequestParts<'_> {
    fn to_capturer(&self) -> HeadersCapturer {
        HeadersCapturer {
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
