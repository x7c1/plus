mod file;
pub use file::FileRequest;

use crate::core::{S3Client, S3Result};
use crate::internal::{InternalClient, InternalRequest, RequestResource};
use crate::verbs::{HasObjectKey, ToEndpoint};
use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use reqwest::{Method, Url};
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

        let fragment = CanonicalFragment {
            url: provider.url,
            method: Method::PUT,
            hashed_payload: resource.hash,
        };
        let request = S3Request {
            fragment: &fragment,
            requested_at: now(),
        };
        let factory = AuthorizationFactory::new(provider.credentials, &request);

        // todo:
        let headers: HeaderMap = HeaderMap::new().authorize_with(factory)?;

        Ok(InternalRequest {
            url: fragment.url,
            method: fragment.method,
            body: resource.body,
            headers,
        })
    }
}

struct S3Request<'a> {
    fragment: &'a CanonicalFragment,
    requested_at: DateTime<Utc>,
}

impl AuthorizationRequest for S3Request<'_> {
    fn to_canonical_fragment(&self) -> &CanonicalFragment {
        self.fragment
    }

    fn to_scope(&self) -> CredentialScope {
        CredentialScope::v4(
            DateStamp::from(&self.requested_at),
            // todo:
            RegionCode::ApNorthEast1,
            ServiceCode::S3,
        )
    }

    fn to_datetime(&self) -> &DateTime<Utc> {
        &self.requested_at
    }
}
