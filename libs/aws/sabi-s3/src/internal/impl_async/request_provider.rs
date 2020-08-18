use super::InternalRequest;
use crate::core::request::ResourceLoader;
use crate::core::verbs::{HasBucketScope, HasMethod, HasObjectKey, ToEndpoint};
use crate::internal;
use crate::internal::Error::RegionNotSpecified;
use crate::internal::RequestParts;
use plus_aws::auth::v4::request::AuthorizationFactory;
use plus_aws::auth::Credentials;
use plus_aws::http::request::{header, RichHeaderMap};
use plus_aws::index::RegionCode;
use reqwest::header::HeaderMap;
use reqwest::{Method, Url};

pub struct RequestProvider<'a, A>
where
    A: ResourceLoader,
    A: HasObjectKey,
{
    credentials: &'a Credentials,
    url: Url,
    method: Method,
    resource_loader: &'a A,
    default_region: &'a Option<RegionCode>,
}

impl<A> RequestProvider<'_, A>
where
    A: ResourceLoader,
    A: HasObjectKey,
{
    pub fn new<'a, X, ANY>(scope: &'a X, request: &'a A) -> internal::Result<RequestProvider<'a, A>>
    where
        X: HasBucketScope,
        A: HasMethod<ANY>,
    {
        let provider = RequestProvider {
            credentials: scope.credentials(),
            url: (scope.bucket(), request).to_endpoint()?,
            method: A::METHOD,
            resource_loader: request,
            default_region: scope.default_region(),
        };
        Ok(provider)
    }

    pub async fn provide(self) -> internal::Result<InternalRequest> {
        let resource = self.resource_loader.load().await?;
        let region_code = resource
            .region
            .or_else(|| self.default_region.as_ref())
            .ok_or_else(|| RegionNotSpecified)?;

        let parts = RequestParts::new(
            self.url,
            self.method,
            &region_code,
            resource.hash,
            resource.requested_at,
        );
        let factory = AuthorizationFactory::new(self.credentials, &parts);
        let headers: HeaderMap = HeaderMap::new()
            .host(&parts.url)?
            .push_if_exists(resource.content_type)?
            .push(header::ContentLength::new(resource.content_length))?
            .push(header::AmzContentSha256::new(parts.hashed_payload.as_str()))?
            .push(header::AmzDate::new(factory.amz_date().as_str()))?
            .authorize_with(factory)?
            .push(header::Date::from(&parts.requested_at))?;

        Ok(InternalRequest {
            url: parts.url,
            method: parts.method,
            body: resource.body,
            headers,
        })
    }
}
