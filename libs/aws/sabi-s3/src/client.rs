use crate::operations;
use crate::S3Result;

#[derive(Debug)]
pub struct S3Client {
    // todo: add bucket field
}

impl S3Client {
    pub async fn put_object<A>(&self, request: A) -> S3Result<String>
    where
        A: operations::put_object::Request,
    {
        operations::put_object::Requester::put_object(self, request)
    }
}
