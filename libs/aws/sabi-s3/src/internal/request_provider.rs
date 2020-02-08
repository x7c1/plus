use crate::core::{S3Bucket, S3Result};
use crate::internal::{InternalRequest, RequestParts, ResourceProvider};
use crate::verbs::{HasObjectKey, ToEndpoint};
use reqwest::header::HeaderMap;
use reqwest::{Method, Url};
use sabi_core::auth::v4::chrono::now;
use sabi_core::auth::v4::request::AuthorizationFactory;
use sabi_core::auth::Credentials;
use sabi_core::http::{header, Headers};

pub struct RequestProvider<'a, A>
where
    A: ResourceProvider,
    A: HasObjectKey,
{
    credentials: &'a Credentials,
    url: Url,
    method: Method,
    resource: A,
}

impl<A> RequestProvider<'_, A>
where
    A: ResourceProvider,
    A: HasObjectKey,
{
    pub fn put<'a>(
        credentials: &'a Credentials,
        bucket: &'a S3Bucket,
        request: A,
    ) -> S3Result<RequestProvider<'a, A>> {
        let provider = RequestProvider {
            credentials,
            url: (bucket, &request).to_endpoint()?,
            method: Method::PUT,
            resource: request,
        };
        Ok(provider)
    }
}

impl<A> From<RequestProvider<'_, A>> for S3Result<InternalRequest>
where
    A: ResourceProvider,
    A: HasObjectKey,
{
    fn from(provider: RequestProvider<A>) -> Self {
        let resource = provider.resource.provide()?;
        let parts = RequestParts::new(
            provider.url,
            provider.method,
            resource.region,
            resource.hash,
            now(),
        );
        let factory = AuthorizationFactory::new(provider.credentials, &parts);
        let headers: HeaderMap = HeaderMap::new()
            .push(resource.content_type)?
            .push(header::AmzContentSha256::new(parts.hashed_payload.as_str()))?
            .push(header::AmzDate::new(factory.amz_date().as_str()))?
            .authorize_with(factory)?;

        Ok(InternalRequest {
            url: parts.url,
            method: parts.method,
            body: resource.body,
            headers,
        })
    }
}
