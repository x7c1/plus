mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;
pub use file::{Outfile, OutfileError};

use crate::actions::get_object;
use crate::client::S3Client;
use crate::core::verbs::HasObjectKey;
use crate::core::{ETag, S3HeaderMap};
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::{actions, core, internal};
use reqwest::header::HeaderMap;
use reqwest::Method;
use sabi_core::io::BodyReceiver;

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
            let provider = RequestProvider::new(Method::GET, &self, &request)?;
            let response: reqwest::blocking::Response = client.request_by(provider)?;
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
