use crate::operations;

#[derive(Debug)]
pub struct S3Client {}

impl S3Client {
    pub fn put_object<A: operations::put_object::Request>(&self, request: A) -> String {
        operations::put_object::Requester::put_object(self, request)
    }
}
