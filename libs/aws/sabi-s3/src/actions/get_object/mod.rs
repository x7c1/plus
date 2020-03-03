mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;
pub use file::{Outfile, OutfileError};

use crate::actions::get_object;
use crate::client::S3Client;
use crate::core::request::ResourceLoader;
use crate::core::response::headers::{AwsHeaderMap, ETag};
use crate::core::verbs::{HasObjectKey, IsGet};
use crate::internal::impl_async::{InternalClient, RequestProvider};
use crate::{actions, core};
use futures_util::TryStreamExt;
use reqwest::header::HeaderMap;
use sabi_core::io::BodyReceiver;

/// rf.
/// [GetObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)
pub trait Request:
    HasObjectKey + ResourceLoader + BodyReceiver<Err = get_object::Error> + Send + Sync
{
}

#[derive(Debug)]
pub struct Response {
    pub headers: Headers,
}

#[derive(Debug)]
pub struct Headers {
    pub e_tag: ETag,
}

impl<A: Request> IsGet<Response> for A {}

type Result<A> = std::result::Result<A, get_object::Error>;

#[async_trait]
pub trait Requester {
    async fn get_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send;
}

#[async_trait]
impl Requester for S3Client {
    async fn get_object<A>(&self, mut request: A) -> actions::Result<Response>
    where
        A: Request,
        A: Send,
    {
        let client = InternalClient::new();
        let headers: get_object::Result<Headers> = async {
            let provider = RequestProvider::new(&self, &request)?;
            let response = client.request_by(provider).await?;
            let headers = to_headers(response.headers())?;
            let stream = response
                .bytes_stream()
                .map_err(|e| get_object::Error::from(e));

            request.receive_body_from(stream).await?;
            Ok(headers)
        }
        .await;
        Ok(Response { headers: headers? })
    }
}

fn to_headers(map: &HeaderMap) -> core::Result<Headers> {
    Ok(Headers {
        e_tag: map.as_required()?,
    })
}
