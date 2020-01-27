mod file_body;

use crate::internal::InternalClient;
use crate::{S3Client, S3Result};
pub use file_body::FileBody;

use std::fmt::Debug;

pub trait Request: Debug {}

pub trait Requester {
    fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: Request;
}

impl Requester for S3Client {
    fn put_object<A: Request>(&self, request: A) -> S3Result<String> {
        let client = InternalClient::new(&self.bucket);
        client.post(request)
    }
}
