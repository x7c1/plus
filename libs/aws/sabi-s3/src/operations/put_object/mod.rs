mod file;
pub use file::FileRequest;

use crate::core::{S3Client, S3Result};
use crate::internal::{InternalClient, InternalRequest, RequestResource};
use crate::verbs::{HasObjectKey, ToEndpoint};
use reqwest::Method;
use url::Url;

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
            url: (&self.bucket, &request).to_endpoint()?,
            original: request,
        };
        client.send(provider)
    }
}

struct RequestProvider<A>
where
    A: Request,
{
    url: Url,
    original: A,
}

impl<A> From<RequestProvider<A>> for S3Result<InternalRequest>
where
    A: Request,
{
    fn from(provider: RequestProvider<A>) -> Self {
        let resource = provider.original.into()?;
        Ok(InternalRequest {
            url: provider.url,
            method: Method::PUT,
            body: resource.body,
            // todo:
            headers: Default::default(),
        })
    }
}
