mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;
pub use file::{Outfile, OutfileError};

use crate::actions::get_object;
use crate::client::S3Client;
use crate::core::response::headers::{ETag, S3HeaderMap};
use crate::core::verbs::{HasObjectKey, IsGet};
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::{actions, core, internal};
use reqwest::header::HeaderMap;
use sabi_core::io::BodyReceiver;

/// rf.
/// [GetObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)
pub trait Request: HasObjectKey + ResourceLoader + BodyReceiver {}

#[derive(Debug)]
pub struct Response {
    pub headers: Headers,
}

#[derive(Debug)]
pub struct Headers {
    pub e_tag: ETag,
}

impl<A: Request> IsGet<Response> for A {}

pub trait Requester {
    fn get_object<A>(&self, request: A) -> actions::Result<Response>
    where
        A: Request;
}

impl Requester for S3Client {
    fn get_object<A>(&self, mut request: A) -> actions::Result<Response>
    where
        A: Request,
    {
        let client = InternalClient::new();
        (|| {
            let provider = RequestProvider::new(&self, &request)?;
            let response = client.request_by(provider)?;
            let headers = to_headers(response.headers())?;

            request.receive_body_from(response)?;
            Ok(Response { headers })
        })()
        .map_err(|e: internal::Error| get_object::Error::from(e).into())
    }
}

fn to_headers(map: &HeaderMap) -> core::Result<Headers> {
    Ok(Headers {
        e_tag: map.get_e_tag()?,
    })
}
