mod file;
pub use file::FileRequest;

use crate::core::{ETag, S3Client, S3Result};
use crate::internal;
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::verbs::HasObjectKey;
use reqwest::header::HeaderMap;
use reqwest::Method;

/// [GetObject - Amazon Simple Storage Service](https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html)
pub trait Request: HasObjectKey + ResourceLoader {}

#[derive(Debug)]
pub struct Response {
    pub e_tag: ETag,
}

pub trait Requester {
    fn get_object<A>(&self, request: A) -> S3Result<Response>
    where
        A: Request;
}

impl Requester for S3Client {
    fn get_object<A>(&self, request: A) -> S3Result<Response>
    where
        A: Request,
    {
        let client = InternalClient::new();
        let provider = RequestProvider::new(
            Method::GET,
            &self.credentials,
            &self.bucket,
            request,
            &self.default_region,
        )?;
        let raw: reqwest::blocking::Response = client.request_by(provider)?;
        let headers: &HeaderMap = raw.headers();

        let text = raw.text()?;
        println!("text...{}", text);

        unimplemented!()
    }
}
