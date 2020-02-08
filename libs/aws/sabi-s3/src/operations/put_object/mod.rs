mod file;
pub use file::FileRequest;

use crate::core::{S3Client, S3Result};
use crate::internal::{InternalClient, RequestProvider, ResourceProvider};
use crate::verbs::HasObjectKey;

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
        let provider = RequestProvider::put(&self.credentials, &self.bucket, request)?;
        client.request_by(provider)
    }
}
