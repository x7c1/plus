mod file;
pub use file::FileRequest;

use crate::internal::{InternalClient, InternalRequest};
use crate::verbs::{HasObjectKey, ToEndpoint};
use crate::{S3Client, S3Result};
use reqwest::blocking::Body;
use url::Url;

pub trait Request: HasObjectKey + Into<S3Result<Body>> {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A: Request>(&self, request: A) -> S3Result<String> {
        let client = InternalClient::new();
        let factory = RequestFactory {
            url: (&self.bucket, &request).to_endpoint()?,
            request,
        };
        client.put(factory)
    }
}

struct RequestFactory<A: Request> {
    url: Url,
    request: A,
}

impl<A> From<RequestFactory<A>> for S3Result<InternalRequest>
where
    A: Request,
{
    fn from(factory: RequestFactory<A>) -> Self {
        Ok(InternalRequest {
            url: factory.url,
            body: Some(factory.request.into()?),
            // todo:
            headers: Default::default(),
        })
    }
}
