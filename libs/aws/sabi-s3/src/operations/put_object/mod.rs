mod file;
pub use file::FileRequest;

use crate::core::{S3Client, S3Result};
use crate::internal::{InternalClient, InternalRequest, RequestResource};
use crate::verbs::{HasObjectKey, ToEndpoint};
use reqwest::header::HeaderMap;
use reqwest::{Method, Url};
use sabi_core::auth::v4::chrono::now;
use sabi_core::auth::v4::request::{AuthorizationFactory, CanonicalFragment};
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
        let time = now();
        let factory = AuthorizationFactory::new(
            provider.credentials,
            &fragment,
            RegionCode::ApNorthEast1,
            ServiceCode::Iam,
            &time,
        );
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
