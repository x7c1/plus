mod file;
pub use file::FileRequest;

use crate::core::{ETag, S3Client, S3HeaderMap, S3Result};
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::verbs::HasObjectKey;
use reqwest::header::HeaderMap;
use reqwest::Method;
use sabi_core::io::Copier;
use std::io::Write;

/// [GetObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)
pub trait Request: HasObjectKey + ResourceLoader + Write {}

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
        A: Request;
}

impl Requester for S3Client {
    fn get_object<A>(&self, mut request: A) -> S3Result<Response>
    where
        A: Request,
    {
        let client = InternalClient::new();
        let provider = RequestProvider::new(
            Method::GET,
            &self.credentials,
            &self.bucket,
            &request,
            &self.default_region,
        )?;
        let response: reqwest::blocking::Response = client.request_by(provider)?;
        let header_map: &HeaderMap = response.headers();
        let headers = Headers {
            e_tag: header_map.get_e_tag()?,
        };
        response.copy_to(&mut request)?;
        Ok(Response { headers })
    }
}
