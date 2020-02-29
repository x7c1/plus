mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;

mod rich_file;
pub use rich_file::RichFile;

use crate::actions::put_object;
use crate::client::S3Client;
use crate::core::response::headers::{AwsHeaderMap, ETag};
use crate::core::verbs::{HasObjectKey, IsPut};
use crate::internal::impl_async;
use crate::{actions, core};
use reqwest::header::HeaderMap;

/// rf.
/// [PutObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)
pub trait Request: HasObjectKey + impl_async::ResourceLoader + Send + Sync {}

#[derive(Debug)]
pub struct Response {
    pub headers: Headers,
}

#[derive(Debug)]
pub struct Headers {
    pub e_tag: ETag,
}

impl<A: Request> IsPut<Response> for A {}

#[async_trait]
pub trait Requester {
    async fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send,
        A: Sync;
}

#[async_trait]
impl Requester for S3Client {
    async fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send,
        A: Sync,
    {
        let client = impl_async::InternalClient::new();
        let provider = impl_async::RequestProvider::new(&self, &request)
            .map_err(|e| put_object::Error::from(e))?;

        let response = client
            .request_by(provider)
            .await
            .map_err(|e| put_object::Error::from(e))?;

        let headers = to_headers(response.headers()).map_err(|e| put_object::Error::from(e))?;
        Ok(Response { headers })
    }
}

fn to_headers(map: &HeaderMap) -> core::Result<Headers> {
    Ok(Headers {
        e_tag: map.as_required()?,
    })
}
