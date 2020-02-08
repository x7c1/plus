mod file;
pub use file::FileRequest;

use crate::core::{S3Client, S3Result};
use crate::internal::{InternalClient, RequestProvider, ResourceProvider};
use crate::verbs::HasObjectKey;
use reqwest::Method;

pub trait Request: HasObjectKey + ResourceProvider {}

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
        let provider = RequestProvider::new(
            Method::PUT,
            &self.credentials,
            &self.bucket,
            request,
            &self.default_region,
        )?;
        client.request_by(provider)
    }
}
