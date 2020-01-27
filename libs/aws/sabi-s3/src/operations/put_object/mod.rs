mod file;
pub use file::FileRequest;

use crate::internal::InternalClient;
use crate::verbs::HasObjectKey;
use crate::{S3Client, S3Result};

pub trait Request: HasObjectKey {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A: Request>(&self, request: A) -> S3Result<String> {
        let client = InternalClient::new(&self.bucket);
        client.put((&self.bucket, request))
    }
}
