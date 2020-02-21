mod file;
pub use file::FileRequest;

mod rich_file;
pub use rich_file::RichFile;

use crate::core::{ETag, S3Client, S3HeaderMap, S3Result};
use crate::internal::{InternalClient, RequestProvider, ResourceLoader};
use crate::verbs::HasObjectKey;
use reqwest::header::HeaderMap;
use reqwest::Method;

pub trait Request: HasObjectKey + ResourceLoader {}

#[derive(Debug)]
pub struct Response {
    pub headers: Headers,
}

#[derive(Debug)]
pub struct Headers {
    pub e_tag: ETag,
}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<Response>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A>(&self, request: A) -> S3Result<Response>
    where
        A: Request,
    {
        let client = InternalClient::new();
        let provider = RequestProvider::new(
            Method::PUT,
            &self.credentials,
            &self.bucket,
            &request,
            &self.default_region,
        )?;
        let raw = client.request_by(provider)?;

        let header_map: &HeaderMap = raw.headers();
        let headers = Headers {
            e_tag: header_map.get_e_tag()?,
        };
        Ok(Response { headers })
    }
}
