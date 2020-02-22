mod error;
pub use error::Error;

mod file;
pub use file::FileRequest;
pub use file::{Outfile, OutfileError};

use crate::core;
use crate::core::{ETag, S3HeaderMap, S3Result};
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::operations::{get_object, S3Client};
use crate::verbs::HasObjectKey;
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
    fn get_object<A>(&self, request: A) -> S3Result<Response>
    where
        A: Request,
        crate::Error: From<<A as BodyReceiver>::Err>;
}

impl Requester for S3Client {
    fn get_object<A>(&self, mut request: A) -> S3Result<Response>
    where
        A: Request,
        crate::Error: From<<A as BodyReceiver>::Err>,
    {
        let client = InternalClient::new();
        let provider = RequestProvider::new(
            Method::GET,
            &self.credentials,
            &self.bucket,
            &request,
            &self.default_region,
        )?;
        let response: reqwest::blocking::Response = client
            .request_by(provider)
            .map_err(|e| get_object::Error::from(e))?;

        let headers = to_headers(response.headers()).map_err(|e| get_object::Error::from(e))?;
        request.receive_body_from(response)?;
        Ok(Response { headers })
    }
}

fn to_headers(map: &HeaderMap) -> core::Result<Headers> {
    Ok(Headers {
        e_tag: map.get_e_tag()?,
    })
}
