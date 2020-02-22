use crate::core::verbs::{HasObjectKey, ToEndpoint};
use crate::core::S3Bucket;
use crate::internal;
use crate::internal::Error::RegionNotSpecified;
use crate::internal::{InternalRequest, RequestParts, ResourceLoader};
use reqwest::header::HeaderMap;
use reqwest::{Method, Url};
use sabi_core::auth::v4::request::AuthorizationFactory;
use sabi_core::auth::Credentials;
use sabi_core::http::header;
use sabi_core::http::RichHeaderMap;
use sabi_core::index::RegionCode;

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
    pub fn new<'a>(
        method: Method,
        credentials: &'a Credentials,
        bucket: &'a S3Bucket,
        request: &'a A,
        default_region: &'a Option<RegionCode>,
    ) -> internal::Result<RequestProvider<'a, A>> {
        let provider = RequestProvider {
            credentials,
            url: (bucket, request).to_endpoint()?,
            method,
            resource_loader: request,
            default_region,
        };
        Ok(provider)
    }

    pub fn provide(self) -> internal::Result<InternalRequest> {
        let resource = self.resource_loader.load()?;
        let region_code = resource
            .region
            .or(self.default_region.as_ref())
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
