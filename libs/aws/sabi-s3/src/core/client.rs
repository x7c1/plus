use crate::core::{S3Bucket, S3Result};
use crate::operations;

#[derive(Debug)]
pub struct S3Client {
    pub bucket: S3Bucket,
}

impl S3Client {
    pub async fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: operations::put_object::Request,
    {
        operations::put_object::Requester::put_object(self, request)
    }
}
