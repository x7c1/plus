mod file;
pub use file::FileRequest;

use crate::core::{S3Client, S3Result};
use crate::internal::{InternalClient, InternalRequest, RequestResource};
use crate::verbs::{HasObjectKey, ToEndpoint};
use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use reqwest::{Method, Url};
use sabi_core::auth::v4::canonical::HashedPayload;
use sabi_core::auth::v4::chrono::{now, DateStamp};
use sabi_core::auth::v4::request::{AuthorizationFactory, AuthorizationRequest, CanonicalFragment};
use sabi_core::auth::v4::sign::CredentialScope;
use sabi_core::auth::Credentials;
use sabi_core::http::Headers;
use sabi_core::index::{RegionCode, ServiceCode};

pub trait Request: HasObjectKey + Into<S3Result<RequestResource>> {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request,
    {
        let client = InternalClient::new();
        let provider = RequestProvider {
            credentials: &self.credentials,
            url: (&self.bucket, &request).to_endpoint()?,
            original: request,
        };
        client.send(provider)
    }
}

struct RequestProvider<'a, A>
where
    A: Request,
{
    credentials: &'a Credentials,
    url: Url,
    original: A,
}

impl<A> From<RequestProvider<'_, A>> for S3Result<InternalRequest>
where
    A: Request,
{
    fn from(provider: RequestProvider<A>) -> Self {
        let resource: RequestResource = provider.original.into()?;
        let request = RequestParts::new(
            provider.url,
            Method::PUT,

            // todo
            RegionCode::ApNorthEast1,
            resource.hash,
            now(),
        );
        let factory = AuthorizationFactory::new(provider.credentials, &request);

        // todo:
        let headers: HeaderMap = HeaderMap::new().authorize_with(factory)?;

        Ok(InternalRequest {
            url: request.url,
            method: request.method,
            body: resource.body,
            headers,
        })
    }
}

struct RequestParts {
    url: Url,
    method: Method,
    requested_at: DateTime<Utc>,
    scope: CredentialScope,
    hashed_payload: HashedPayload,
}
impl RequestParts {
    fn new(
        url: Url,
        method: Method,
        region: RegionCode,
        hashed_payload: HashedPayload,
        requested_at: DateTime<Utc>,
    ) -> RequestParts {
        let date = DateStamp::from(&requested_at);
        RequestParts {
            url,
            method,
            requested_at,
            scope: CredentialScope::v4(date, region, ServiceCode::S3),
            hashed_payload,
        }
    }
}

impl AuthorizationRequest for RequestParts {
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
