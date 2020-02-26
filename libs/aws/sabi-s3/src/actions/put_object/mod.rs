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
use crate::internal::blocking::{InternalClient, RequestProvider, ResourceLoader};
use crate::{actions, core, internal};
use reqwest::header::HeaderMap;

/// rf.
/// [PutObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)
pub trait Request: HasObjectKey + ResourceLoader {}

#[derive(Debug)]
pub struct Response {
    pub headers: Headers,
}

#[derive(Debug)]
pub struct Headers {
    pub e_tag: ETag,
}

impl<A: Request> IsPut<Response> for A {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request,
    {
        let client = InternalClient::new();
        (|| {
            let provider = RequestProvider::new(&self, &request)?;
            let response = client.request_by(provider)?;
            let headers = to_headers(response.headers())?;
            Ok(Response { headers })
        })()
        .map_err(|e: internal::Error| put_object::Error::from(e).into())
    }
}

fn to_headers(map: &HeaderMap) -> core::Result<Headers> {
    Ok(Headers {
        e_tag: map.as_required()?,
    })
}
